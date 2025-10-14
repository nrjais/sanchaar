use lib::persistence::environment::{encode_environments, save_environments};
use lib::persistence::{ENVIRONMENTS, REQUESTS, TOML_EXTENSION};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use iced::Task;
use iced::futures::TryFutureExt;
use rfd::{AsyncFileDialog, FileHandle};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use lib::client::send_request;
use lib::http::collection::{Collection, Entry, FolderId, RequestId, RequestRef};
use lib::http::request::Request;
use lib::http::{CollectionKey, CollectionRequest, Environment, EnvironmentKey, KeyValList};
use lib::persistence::collections::{self, encode_collection, open_collection, save_collection};
use lib::persistence::history::HistoryDatabase;
use lib::persistence::request::{encode_request, read_request, save_req_to_file};
use lib::transformers::request::transform_request;

use crate::state::response::ResponseState;
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::utils::to_core_kv_list;
use crate::state::{AppState, CommonState, HttpTab, RequestDirtyState, Tab, TabKey};

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Completed(lib::client::Response),
    Error(Arc<anyhow::Error>),
}

pub fn send_request_cmd(state: &mut CommonState, tab: &mut HttpTab) -> Task<ResponseResult> {
    let collection = state.collections.get(tab.collection_ref.0);

    let mut request = tab.request().to_request();
    if let Some(col) = collection {
        let mut headers = KeyValList::clone(&col.headers);
        headers.extend(request.headers);
        request.headers = headers;
    }

    let env = collection.map(|c| c.env_chain()).unwrap_or_default();
    let disable_ssl = collection.map(|c| c.disable_ssl).unwrap_or_default();
    let client = if disable_ssl {
        state.client_no_ssl.clone()
    } else {
        state.client.clone()
    };

    let history_db = state.history_db.clone();
    let collection_name = collection.map(|c| c.name.clone());
    let request_for_history = request.clone();

    let req_fut = transform_request(client.clone(), request, env)
        .and_then(move |req| send_request(client, req))
        .and_then(move |response| async move {
            if let Some(db) = history_db
                && let Err(e) = save_request_to_history(
                    &db,
                    &request_for_history,
                    &response,
                    collection_name.as_deref(),
                )
                .await
            {
                log::error!("Failed to save request to history: {e}");
            }
            Ok(response)
        });

    tab.cancel_tasks();
    tab.response.state = ResponseState::Executing;

    let (task, handle) = Task::perform(req_fut, move |r| match r {
        Ok(res) => ResponseResult::Completed(res),
        Err(e) => ResponseResult::Error(Arc::new(e)),
    })
    .abortable();
    tab.add_task(handle);

    task
}

async fn save_request_to_history(
    history_db: &HistoryDatabase,
    request: &Request,
    response: &lib::client::Response,
    collection_name: Option<&str>,
) -> anyhow::Result<()> {
    history_db
        .save_request_response(request, response, collection_name)
        .await?;
    Ok(())
}

pub fn save_request_cmd(tab: &mut HttpTab, path: PathBuf) -> Task<Option<Arc<anyhow::Error>>> {
    tab.mark_clean();

    let encoded = encode_request(tab.request().to_request());
    Task::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => None,
        Err(e) => {
            log::error!("Error saving request: {e:?}");
            Some(Arc::new(e))
        }
    })
}

pub fn write_file_cmd(data: Arc<Vec<u8>>, path: Arc<FileHandle>) -> Task<()> {
    async fn fut(data: Arc<Vec<u8>>, path: Arc<FileHandle>) -> anyhow::Result<()> {
        let mut file = fs::File::create(path.path()).await?;
        file.write_all(&data).await?;
        Ok(())
    }

    Task::perform(fut(data, path), move |r| match r {
        Ok(_) => (),
        Err(e) => log::error!("Error saving file: {e:?}"),
    })
}

pub fn save_tab_request_cmd(
    state: &mut AppState,
    name: String,
    tab: TabKey,
    col: CollectionKey,
    fol: Option<FolderId>,
) -> Task<Option<anyhow::Error>> {
    let Some(Tab::Http(tab)) = state.get_tab_mut(tab) else {
        return Task::none();
    };
    tab.mark_clean();
    let req = tab.request().to_request();

    create_new_request_cmd(&mut state.common, col, fol, name, req)
}

pub fn create_new_request_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    fol: Option<FolderId>,
    name: String,
    req: Request,
) -> Task<Option<anyhow::Error>> {
    let Some(collection) = state.collections.get_mut(col) else {
        return Task::none();
    };

    let path = match fol {
        Some(fol) => {
            let Some(folder) = collection.folder_mut(fol) else {
                return Task::none();
            };
            let path = folder.path.join(format!("{}{}", &name, TOML_EXTENSION));
            folder.entries.push(Entry::Item(RequestRef {
                name,
                id: RequestId::new(),
                path: path.clone(),
            }));
            path
        }
        None => {
            let path = collection
                .path
                .join(REQUESTS)
                .join(format!("{}{}", &name, TOML_EXTENSION));
            collection.entries.push(Entry::Item(RequestRef {
                name,
                id: RequestId::new(),
                path: path.clone(),
            }));
            path
        }
    };

    let encoded = encode_request(req);

    Task::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => None,
        Err(e) => {
            log::error!("Error saving request: {e:?}");
            Some(e)
        }
    })
}

pub fn create_collection_cmd(
    state: &mut CommonState,
    name: String,
    path: PathBuf,
) -> Task<Option<anyhow::Error>> {
    let col = state.collections.create_collection(name, path);
    let encoded = encode_collection(col);
    Task::perform(
        save_collection(col.path.clone(), encoded),
        move |r| match r {
            Ok(_) => None,
            Err(e) => {
                log::error!("Error saving collection: {e:?}");
                Some(e)
            }
        },
    )
}

pub fn open_collection_cmd() -> Task<Option<Collection>> {
    let fut = async {
        let handle = AsyncFileDialog::new()
            .set_title("Select Collection Folder")
            .pick_folder()
            .await?;

        let path = handle.path().to_owned();
        let key = CollectionKey::new();

        let col = open_collection(path, key).await.ok()?;

        Some(col)
    };

    Task::perform(fut, |r| r)
}

pub fn open_request_cmd(
    state: &mut CommonState,
    col: CollectionRequest,
) -> Task<Option<(Request, String)>> {
    let Some(req) = state.collections.get_ref(col) else {
        return Task::none();
    };

    let path = req.path.clone();
    let name = req.name.clone();

    let fut = async move { read_request(&path).await };

    Task::perform(fut, move |res| match res {
        Ok(req) => Some((req, name.clone())),
        Err(e) => {
            log::error!("Error opening request: {:?}", &e);
            None
        }
    })
}

pub fn delete_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: FolderId,
) -> Task<()> {
    let path = state.collections.delete_folder(col, folder_id);
    if let Some(path) = path {
        Task::perform(fs::remove_dir_all(path), move |_| ())
    } else {
        Task::none()
    }
}

pub fn create_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: Option<FolderId>,
    name: String,
) -> Task<()> {
    let path = state.collections.create_folder_in(name, col, folder_id);

    if let Some(path) = path {
        Task::perform(fs::create_dir_all(path), move |_| ())
    } else {
        Task::none()
    }
}

pub fn create_script_cmd(state: &mut CommonState, col: CollectionKey, name: String) -> Task<()> {
    let Some(path) = state.collections.create_script_in(col, name) else {
        return Task::none();
    };

    let fut = || async {
        let parent = path.parent();
        if let Some(parent) = parent {
            fs::create_dir_all(parent).await?;
        }
        fs::File::create(path).await?;
        anyhow::Ok(())
    };

    Task::perform(fut(), move |_| ())
}

pub fn save_environments_cmd(
    collection: &mut Collection,
    envs: HashMap<EnvironmentKey, Environment>,
) -> Task<()> {
    let removed = collection.replace_environments(envs);

    let encoded = encode_environments(&collection.environments);
    let mut delete_path = Vec::new();

    for env in removed {
        delete_path.push(
            collection
                .path
                .join(ENVIRONMENTS)
                .join(format!("{}{}", env.name, TOML_EXTENSION)),
        );
    }

    let delete_fut = async {
        for path in delete_path {
            fs::remove_file(path).await?;
        }
        Ok(())
    };
    let fut = save_environments(collection.path.clone(), encoded).and_then(|_| delete_fut);

    Task::perform(fut, move |_| ())
}

pub async fn load_collections_cmd() -> Vec<Collection> {
    collections::load().await.unwrap_or_else(|e| {
        log::error!("Error loading http: {e:?}");
        vec![]
    })
}

pub fn check_dirty_requests_cmd(state: &mut AppState) -> Task<Vec<(TabKey, RequestDirtyState)>> {
    let mut to_check = Vec::new();
    for (key, tab) in state.tabs.iter_mut() {
        let Tab::Http(tab) = tab else {
            continue;
        };

        if RequestDirtyState::CheckIfDirty != tab.request_dirty_state {
            continue;
        }

        let Some(request_ref) = state.common.collections.get_ref(tab.collection_ref) else {
            tab.request_dirty_state = RequestDirtyState::Clean;
            continue;
        };

        let req = tab.request().to_request();

        to_check.push((*key, req, request_ref.path.clone()));
    }

    async fn exec(
        to_check: Vec<(TabKey, Request, PathBuf)>,
    ) -> Result<Vec<(TabKey, RequestDirtyState)>, anyhow::Error> {
        let mut status = Vec::new();
        for (key, req, path) in to_check {
            let file_request = read_request(&path).await?;
            if req != file_request {
                status.push((key, RequestDirtyState::Dirty));
            } else {
                status.push((key, RequestDirtyState::Clean));
            }
        }

        Ok(status)
    }

    Task::perform(exec(to_check), move |res| match res {
        Ok(dirty) => dirty,
        Err(e) => {
            log::error!("Error checking dirty requests: {e:?}");
            vec![]
        }
    })
}

pub fn rename_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: FolderId,
    name: String,
) -> Task<()> {
    let Some((old, new)) = state.collections.rename_folder(col, folder_id, name) else {
        return Task::none();
    };

    Task::perform(fs::rename(old, new), move |res| {
        if let Err(e) = res {
            log::error!("Error renaming folder: {e:?}");
        }
    })
}

pub fn rename_request_cmd(
    state: &mut CommonState,
    col: CollectionRequest,
    name: String,
) -> Task<()> {
    let Some((old, new)) = state.collections.rename_request(col, name) else {
        return Task::none();
    };

    Task::perform(fs::rename(old, new), move |res| {
        if let Err(e) = res {
            log::error!("Error renaming request: {e:?}");
        }
    })
}

pub fn delete_request_cmd(state: &mut CommonState, col: CollectionKey, req: RequestId) -> Task<()> {
    let Some(path) = state.collections.delete_request(col, req) else {
        return Task::none();
    };

    Task::perform(fs::remove_file(path), move |_| ())
}

pub fn save_collection_cmd(collection: &mut Collection, tab: &mut CollectionTab) -> Task<()> {
    tab.edited = false;
    collection.default_env = tab
        .default_env
        .as_ref()
        .and_then(|name| collection.environments.find_by_name(name));
    collection.headers = Arc::new(to_core_kv_list(&tab.headers));
    collection.disable_ssl = tab.disable_ssl;
    collection.timeout = tab.timeout;

    let encoded = encode_collection(collection);
    Task::perform(
        save_collection(collection.path.clone(), encoded),
        move |r| match r {
            Ok(_) => (),
            Err(e) => {
                log::error!("Error saving collection: {e:?}");
            }
        },
    )
}

pub fn import_postman_collection_cmd(
    _state: &mut CommonState,
    postman_file: PathBuf,
    collection_path: PathBuf,
) -> Task<Option<Collection>> {
    Task::perform(
        import_postman_collection(postman_file, collection_path),
        move |result| match result {
            Ok(collection) => {
                log::info!("Successfully imported Postman collection");
                Some(collection)
            }
            Err(e) => {
                log::error!("Error importing Postman collection: {e:?}");
                None
            }
        },
    )
}
