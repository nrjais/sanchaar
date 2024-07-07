use core::http::environment::EnvironmentKey;
use core::persistence::environment::{encode_environments, save_environments};
use core::persistence::{ENVIRONMENTS, REQUESTS, HCL_EXTENSION, TOML_EXTENSION};
use std::path::PathBuf;
use std::sync::Arc;

use iced::advanced::graphics::futures::MaybeSend;
use iced::futures::TryFutureExt;
use iced::Task;
use rfd::AsyncFileDialog;
use tokio::fs;

use core::client::send_request;
use core::http::collection::Collection;
use core::http::{
    collection::{Entry, FolderId, RequestId, RequestRef},
    request::Request,
    CollectionKey, CollectionRequest,
};
use core::persistence::collections::{self, encode_collection, open_collection, save_collection};
use core::persistence::request::{encode_request, read_request, save_req_to_file};
use core::transformers::request::transform_request;

use crate::commands::cancellable_task::{cancellable_task, TaskResult};
use crate::state::request::RequestPane;
use crate::state::response::ResponseState;
use crate::state::{AppState, RequestDirtyState, TabKey};

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Completed(core::client::Response),
    Error(Arc<anyhow::Error>),
    Cancelled,
}

pub fn send_request_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    tab: TabKey,
    on_result: impl Fn(ResponseResult) -> M + 'static + MaybeSend,
) -> Task<M> {
    let Some(sel_tab) = state.get_tab(tab) else {
        return Task::none();
    };

    let env = match sel_tab.collection_ref {
        Some(col) => state.collections.get_active_env(col.0).cloned(),
        None => None,
    };

    let client = state.client.clone();
    let req_fut = transform_request(client.clone(), sel_tab.request().to_request(), env)
        .and_then(move |req| send_request(client, req));

    let (cancel_tx, req_fut) = cancellable_task(req_fut);

    let Some(sel_tab) = state.get_tab_mut(tab) else {
        return Task::none();
    };
    sel_tab.response.state = ResponseState::Executing;
    sel_tab.add_task(cancel_tx);

    Task::perform(req_fut, move |r| match r {
        TaskResult::Completed(Ok(res)) => on_result(ResponseResult::Completed(res)),
        TaskResult::Cancelled => on_result(ResponseResult::Cancelled),
        TaskResult::Completed(Err(e)) => on_result(ResponseResult::Error(Arc::new(e))),
    })
}

pub fn save_request_cmd<M: 'static + MaybeSend>(
    req: &RequestPane,
    path: PathBuf,
    on_done: impl Fn(Option<Arc<anyhow::Error>>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let encoded = encode_request(req.to_request());
    Task::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => on_done(None),
        Err(e) => {
            log::error!("Error saving request: {:?}", e);
            on_done(Some(Arc::new(e)))
        }
    })
}

pub fn save_tab_request_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    name: String,
    tab: TabKey,
    col: CollectionKey,
    fol: Option<FolderId>,
    msg: impl Fn(Option<anyhow::Error>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let Some(sel_tab) = state.get_tab(tab) else {
        return Task::none();
    };
    let req = sel_tab.request().to_request();

    create_new_request_cmd(state, col, fol, name, req, msg)
}

pub fn create_new_request_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    fol: Option<FolderId>,
    name: String,
    req: Request,
    msg: impl Fn(Option<anyhow::Error>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let Some(collection) = state.collections.get_mut(col) else {
        return Task::none();
    };

    let path = match fol {
        Some(fol) => {
            let Some(folder) = collection.folder_mut(fol) else {
                return Task::none();
            };
            let path = folder.path.join(format!("{}.{}", &name, HCL_EXTENSION));
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
                .join(format!("{}.toml", &name));
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
        Ok(_) => msg(None),
        Err(e) => {
            log::error!("Error saving request: {:?}", e);
            msg(Some(e))
        }
    })
}

pub(crate) fn create_collection_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    name: String,
    path: PathBuf,
    msg: impl Fn(Option<anyhow::Error>) -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let col = state.collections.create_collection(name, path);
    let encoded = encode_collection(col);
    Task::perform(
        save_collection(col.path.clone(), encoded),
        move |r| match r {
            Ok(_) => msg(None),
            Err(e) => {
                log::error!("Error saving collection: {:?}", e);
                msg(Some(e))
            }
        },
    )
}

pub fn open_collection_cmd<M: 'static + MaybeSend>(
    on_done: impl Fn(Option<Collection>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let fut = async {
        let handle = AsyncFileDialog::new()
            .set_title("Select Collection Folder")
            .pick_folder()
            .await?;

        let path = handle.path().to_owned();

        let col = open_collection(path).await.ok()?;

        Some(col)
    };

    Task::perform(fut, on_done)
}

pub fn open_request_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionRequest,
    on_done: impl Fn(Option<Request>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let Some(req) = state.collections.get_ref(col) else {
        return Task::none();
    };

    Task::perform(read_request(req.path.clone()), move |res| match res {
        Ok(req) => on_done(Some(req)),
        Err(e) => {
            log::error!("Error opening request: {:?}", e);
            on_done(None)
        }
    })
}

pub(crate) fn delete_folder_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    folder_id: FolderId,
    on_done: impl Fn() -> M + 'static + MaybeSend,
) -> Task<M> {
    let path = state.collections.delete_folder(col, folder_id);
    if let Some(path) = path {
        Task::perform(fs::remove_dir_all(path), move |_| on_done())
    } else {
        Task::none()
    }
}

pub(crate) fn create_folder_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    folder_id: Option<FolderId>,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let path = state.collections.create_folder_in(name, col, folder_id);

    if let Some(path) = path {
        Task::perform(fs::create_dir_all(path), move |_| done())
    } else {
        Task::none()
    }
}

pub(crate) fn create_script_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
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

    Task::perform(fut(), move |_| done())
}

pub(crate) fn save_environments_cmd<Message: 'static + MaybeSend>(
    collection: &mut Collection,
    deletions: &[EnvironmentKey],
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let encoded = encode_environments(&collection.environments);
    let mut delete_path = Vec::new();

    for key in deletions {
        let env = collection.delete_environment(*key);
        if let Some(env) = env {
            delete_path.push(
                collection
                    .path
                    .join(ENVIRONMENTS)
                    .join(format!("{}{}", env.name, TOML_EXTENSION)),
            );
        }
    }

    let delete_fut = async {
        for path in delete_path {
            fs::remove_file(path).await?;
        }
        Ok(())
    };
    let fut = save_environments(collection.path.clone(), encoded).and_then(|_| delete_fut);

    Task::perform(fut, move |_| done())
}

pub async fn load_collections_cmd() -> Vec<Collection> {
    collections::load().await.unwrap_or_else(|e| {
        log::error!("Error loading http: {:?}", e);
        vec![]
    })
}

pub(crate) fn check_dirty_requests_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    on_done: impl Fn(Vec<(TabKey, RequestDirtyState)>) -> M + 'static + MaybeSend,
) -> Task<M> {
    let mut to_check = Vec::new();
    for (key, tab) in state.tabs.iter_mut() {
        if RequestDirtyState::CheckIfDirty != tab.request_dirty_state {
            continue;
        }
        let Some(col) = tab.collection_ref.as_ref() else {
            tab.request_dirty_state = RequestDirtyState::Clean;
            continue;
        };

        let Some(request_ref) = state.collections.get_ref(*col) else {
            tab.request_dirty_state = RequestDirtyState::Clean;
            continue;
        };

        let req = tab.request().to_request();

        to_check.push((key, req, request_ref.path.clone()));
    }

    async fn exec(
        to_check: Vec<(TabKey, Request, PathBuf)>,
    ) -> Result<Vec<(TabKey, RequestDirtyState)>, anyhow::Error> {
        let mut status = Vec::new();
        for (key, req, path) in to_check {
            let file_request = read_request(path).await?;
            if req != file_request {
                status.push((key, RequestDirtyState::Dirty));
            } else {
                status.push((key, RequestDirtyState::Clean));
            }
        }

        Ok(status)
    }

    Task::perform(exec(to_check), move |res| match res {
        Ok(dirty) => on_done(dirty),
        Err(e) => {
            log::error!("Error checking dirty requests: {:?}", e);
            on_done(vec![])
        }
    })
}

pub(crate) fn rename_collection_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let Some((old, new)) = state.collections.rename_collection(col, name) else {
        return Task::none();
    };

    Task::perform(fs::rename(old, new), move |res| {
        if let Err(e) = res {
            log::error!("Error renaming collection: {:?}", e);
        }
        done()
    })
}

pub(crate) fn rename_folder_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    folder_id: FolderId,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let Some((old, new)) = state.collections.rename_folder(col, folder_id, name) else {
        return Task::none();
    };

    Task::perform(fs::rename(old, new), move |res| {
        if let Err(e) = res {
            log::error!("Error renaming folder: {:?}", e);
        }
        done()
    })
}

pub(crate) fn rename_request_cmd<Message: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionRequest,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Task<Message> {
    let Some((old, new)) = state.collections.rename_request(col, name) else {
        return Task::none();
    };

    Task::perform(fs::rename(old, new), move |res| {
        if let Err(e) = res {
            log::error!("Error renaming request: {:?}", e);
        }
        done()
    })
}

pub(crate) fn delete_request_cmd<M: 'static + MaybeSend>(
    state: &mut AppState,
    col: CollectionKey,
    req: RequestId,
    action: impl Fn() -> M + 'static + MaybeSend,
) -> Task<M> {
    let Some(path) = state.collections.delete_request(col, req) else {
        return Task::none();
    };

    Task::perform(fs::remove_file(path), move |_| action())
}
