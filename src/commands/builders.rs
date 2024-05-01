use std::path::PathBuf;
use std::sync::Arc;

use iced::advanced::graphics::futures::MaybeSend;
use iced::futures::TryFutureExt;
use iced::Command;
use rfd::AsyncFileDialog;
use tokio::fs;

use core::client::send_request;
use core::http::collection::Collection;
use core::http::{
    collection::{Entry, FolderId, RequestId, RequestRef},
    request::Request,
    CollectionKey, CollectionRequest,
};
use core::persistence::collections::{encode_collection, open_collection, save_collection};
use core::persistence::request::{encode_request, read_request, save_req_to_file};
use core::transformers::request::transform_request;

use crate::commands::cancellable_task::{cancellable_task, TaskResult};
use crate::state::request::RequestPane;
use crate::state::response::ResponseState;
use crate::state::{AppState, TabKey};

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Completed(core::client::Response),
    Error(Arc<anyhow::Error>),
    Cancelled,
}

pub fn send_request_cmd<M>(
    state: &mut AppState,
    tab: TabKey,
    on_result: impl Fn(ResponseResult) -> M + 'static + MaybeSend,
) -> Command<M> {
    let client = state.client.clone();
    let Some(sel_tab) = state.get_tab_mut(tab) else {
        return Command::none();
    };

    sel_tab.response.state = ResponseState::Executing;
    let req_fut = transform_request(client.clone(), sel_tab.request.to_request())
        .and_then(|req| send_request(client, req));

    let (cancel_tx, req_fut) = cancellable_task(req_fut);
    sel_tab.add_task(cancel_tx);

    Command::perform(req_fut, move |r| match r {
        TaskResult::Completed(Ok(res)) => on_result(ResponseResult::Completed(res)),
        TaskResult::Cancelled => on_result(ResponseResult::Cancelled),
        TaskResult::Completed(Err(e)) => on_result(ResponseResult::Error(Arc::new(e))),
    })
}

pub fn save_request<M>(
    req: &RequestPane,
    path: PathBuf,
    on_done: impl Fn(Option<Arc<anyhow::Error>>) -> M + 'static + MaybeSend,
) -> Command<M> {
    let encoded = encode_request(req.to_request());
    Command::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => on_done(None),
        Err(e) => {
            println!("Error saving request: {:?}", e);
            on_done(Some(Arc::new(e)))
        }
    })
}

pub fn save_new_request<M>(
    state: &mut AppState,
    name: String,
    tab: TabKey,
    col: CollectionKey,
    fol: Option<FolderId>,
    msg: impl Fn(Option<anyhow::Error>) -> M + 'static + MaybeSend,
) -> Command<M> {
    let Some(collection) = state.collections.get_mut(col) else {
        return Command::none();
    };
    let path = match fol {
        Some(fol) => {
            let Some(folder) = collection.folder_mut(fol) else {
                return Command::none();
            };
            let path = folder.path.join(format!("{}.toml", &name));
            folder.children.push(Entry::Item(RequestRef {
                name,
                id: RequestId::new(),
                path: path.clone(),
            }));
            path
        }
        None => {
            let path = collection.path.join(format!("{}.toml", &name));
            collection.children.push(Entry::Item(RequestRef {
                name,
                id: RequestId::new(),
                path: path.clone(),
            }));
            path
        }
    };

    let Some(sel_tab) = state.get_tab(tab) else {
        return Command::none();
    };

    let req = sel_tab.request.to_request();
    let encoded = encode_request(req);

    Command::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => msg(None),
        Err(e) => {
            println!("Error saving request: {:?}", e);
            msg(Some(e))
        }
    })
}

pub(crate) fn create_collection<Message>(
    state: &mut AppState,
    name: String,
    path: PathBuf,
    msg: impl Fn(Option<anyhow::Error>) -> Message + 'static + MaybeSend,
) -> Command<Message> {
    let col = state.collections.create_collection(name, path);
    let encoded = encode_collection(col);
    Command::perform(
        save_collection(col.path.clone(), encoded),
        move |r| match r {
            Ok(_) => msg(None),
            Err(e) => {
                println!("Error saving collection: {:?}", e);
                msg(Some(e))
            }
        },
    )
}

pub fn open_existing_collection<M>(
    on_done: impl Fn(Option<Collection>) -> M + 'static + MaybeSend,
) -> Command<M> {
    let fut = async {
        let handle = AsyncFileDialog::new()
            .set_title("Select Collection Folder")
            .pick_folder()
            .await?;

        let path = handle.path().to_owned();

        let col = open_collection(path).await.ok()?;

        Some(col)
    };

    Command::perform(fut, on_done)
}

pub fn open_request_cmd<M>(
    state: &mut AppState,
    col: CollectionRequest,
    on_done: impl Fn(Option<Request>) -> M + 'static + MaybeSend,
) -> Command<M> {
    let Some(req) = state.collections.get_ref(col) else {
        return Command::none();
    };

    Command::perform(read_request(req.path.clone()), move |res| match res {
        Ok(req) => on_done(Some(req)),
        Err(e) => {
            println!("Error opening request: {:?}", e);
            on_done(None)
        }
    })
}

pub(crate) fn delete_folder_cmd<M>(
    state: &mut AppState,
    col: CollectionKey,
    folder_id: FolderId,
    on_done: impl Fn() -> M + 'static + MaybeSend,
) -> Command<M> {
    let path = state.collections.delete_folder(col, folder_id);
    if let Some(path) = path {
        Command::perform(fs::remove_dir_all(path), move |_| on_done())
    } else {
        Command::none()
    }
}

pub(crate) fn create_folder<Message>(
    state: &mut AppState,
    col: CollectionKey,
    folder_id: Option<FolderId>,
    name: String,
    done: impl Fn() -> Message + 'static + MaybeSend,
) -> Command<Message> {
    let path = state.collections.create_folder_in(name, col, folder_id);

    if let Some(path) = path {
        Command::perform(fs::create_dir(path), move |_| done())
    } else {
        Command::none()
    }
}
