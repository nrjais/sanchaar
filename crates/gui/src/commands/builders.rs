use core::http::KeyValList;
use std::sync::Arc;

use iced::futures::TryFutureExt;
use iced::Task;
use rfd::FileHandle;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use core::client::send_request;
use core::http::collection::Collection;
use core::http::request::Request;
use core::http::{
    collection::{FolderId, RequestId},
    CollectionKey, CollectionRequest,
};
use core::persistence::collections::{
    create_collection_in_database, load_collections_from_database,
};
use core::transformers::request::transform_request;

use crate::commands::cancellable_task::{cancellable_task, TaskResult};
use crate::state::response::ResponseState;
use crate::state::tabs::collection_tab::{CollectionTab, EnvironmentEditor};
use crate::state::{AppState, CommonState, HttpTab, RequestDirtyState, Tab, TabKey};

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Completed(core::client::Response),
    Error(Arc<anyhow::Error>),
    Cancelled,
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

    let req_fut = transform_request(client.clone(), request, env)
        .map_err(|e| anyhow::anyhow!(e))
        .and_then(move |req| send_request(client, req));

    let (cancel_tx, req_fut) = cancellable_task(req_fut);

    tab.cancel_tasks();
    tab.response.state = ResponseState::Executing;
    tab.add_task(cancel_tx);

    Task::perform(req_fut, move |r| match r {
        TaskResult::Completed(Ok(res)) => ResponseResult::Completed(res),
        TaskResult::Cancelled => ResponseResult::Cancelled,
        TaskResult::Completed(Err(e)) => ResponseResult::Error(Arc::new(e)),
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
        Err(e) => log::error!("Error saving file: {:?}", e),
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
    _req: Request,
) -> Task<Option<anyhow::Error>> {
    let Some(collection) = state.collections.get_mut(col) else {
        return Task::none();
    };

    let _request_id = collection.create_request(name, fol);

    // Note: In database mode, we would save the request to the database here
    // For now, just create the request entry in the collection
    Task::done(None)
}

pub fn create_collection_cmd(
    _state: &mut CommonState,
    name: String,
) -> Task<Option<anyhow::Error>> {
    Task::perform(
        async move {
            tokio::task::spawn_blocking(move || create_collection_in_database(name))
                .await
                .unwrap_or_else(|e| Err(anyhow::anyhow!("Blocking task error: {:?}", e)))
        },
        move |r| match r {
            Ok(collection_id) => {
                log::info!("Created collection with ID: {}", collection_id);
                None
            }
            Err(e) => {
                log::error!("Error creating collection: {:?}", e);
                Some(e)
            }
        },
    )
}

pub fn open_request_cmd(
    _state: &mut CommonState,
    _col: CollectionRequest,
) -> Task<Option<(Request, String)>> {
    // In database mode, we would load the request from the database
    // For now, return None to disable this functionality
    Task::done(None)
}

pub fn delete_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: FolderId,
) -> Task<()> {
    let _deleted = state.collections.delete_folder(col, folder_id);
    // In database mode, we would delete from the database
    Task::done(())
}

pub fn create_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: Option<FolderId>,
    name: String,
) -> Task<()> {
    let _folder_id = state.collections.create_folder_in(name, col, folder_id);
    // In database mode, we would save to the database
    Task::done(())
}

pub fn create_script_cmd(state: &mut CommonState, col: CollectionKey, name: String) -> Task<()> {
    let _script_name = state.collections.create_script_in(col, name, String::new());
    // In database mode, we would save to the database
    Task::done(())
}

pub fn save_environments_cmd(
    collection: &mut Collection,
    data: &mut EnvironmentEditor,
) -> Task<()> {
    data.edited = false;
    for (key, env) in data.environments.iter() {
        collection.update_environment(*key, env.into());
    }

    for key in &data.deleted {
        collection.delete_environment(*key);
    }

    // In database mode, we would save environments to the database
    Task::done(())
}

pub fn load_collections_cmd() -> Task<Vec<Collection>> {
    Task::perform(
        async {
            tokio::task::spawn_blocking(|| {
                load_collections_from_database().unwrap_or_else(|e| {
                    log::error!("Error loading collections: {:?}", e);
                    vec![]
                })
            })
            .await
            .unwrap_or_else(|e| {
                log::error!("Error in blocking task: {:?}", e);
                vec![]
            })
        },
        |collections| collections,
    )
}

pub fn check_dirty_requests_cmd(_state: &mut AppState) -> Task<Vec<(TabKey, RequestDirtyState)>> {
    // In database mode, we would check dirty state differently
    // For now, assume all requests are clean
    Task::done(vec![])
}

pub fn rename_folder_cmd(
    state: &mut CommonState,
    col: CollectionKey,
    folder_id: FolderId,
    name: String,
) -> Task<()> {
    let _success = state.collections.rename_folder(col, folder_id, name);
    // In database mode, we would update the database
    Task::done(())
}

pub fn rename_request_cmd(
    state: &mut CommonState,
    col: CollectionRequest,
    name: String,
) -> Task<()> {
    let _success = state.collections.rename_request(col, name);
    // In database mode, we would update the database
    Task::done(())
}

pub fn delete_request_cmd(state: &mut CommonState, col: CollectionKey, req: RequestId) -> Task<()> {
    let _success = state.collections.delete_request(col, req);
    // In database mode, we would delete from the database
    Task::done(())
}

pub fn save_collection_cmd(_collection: &mut Collection, tab: &mut CollectionTab) -> Task<()> {
    tab.edited = false;
    // In database mode, we would save the collection to the database
    Task::done(())
}
