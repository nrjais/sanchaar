use std::path::PathBuf;
use std::sync::Arc;

use iced::advanced::graphics::futures::MaybeSend;
use iced::Command;
use iced::futures::TryFutureExt;

use core::client::send_request;
use core::http::{
    collection::{Entry, FolderId, RequestId, RequestRef},
    CollectionKey,
};
use core::persistence::request::{encode_request, save_req_to_file};
use core::transformers::request::transform_request;

use crate::commands::{CommandResultMsg, ResponseResult};
use crate::commands::cancellable_task::{cancellable_task, TaskResult};
use crate::state::{AppState, TabKey};
use crate::state::request::RequestPane;
use crate::state::response::ResponseState;

pub fn send_request_cmd(state: &mut AppState, tab: TabKey) -> Command<CommandResultMsg> {
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
        TaskResult::Completed(Ok(res)) => {
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Completed(res))
        }
        TaskResult::Cancelled => CommandResultMsg::UpdateResponse(tab, ResponseResult::Cancelled),
        TaskResult::Completed(Err(e)) => {
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Error(Arc::new(e)))
        }
    })
}

pub fn save_request(req: &RequestPane, path: PathBuf) -> Command<CommandResultMsg> {
    let encoded = encode_request(&req.to_request());
    Command::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => CommandResultMsg::Completed("Request saved"),
        Err(e) => {
            println!("Error saving request: {:?}", e);
            CommandResultMsg::Completed("Error saving request")
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
    let encoded = encode_request(&req);

    Command::perform(save_req_to_file(path, encoded), move |r| match r {
        Ok(_) => msg(None),
        Err(e) => {
            println!("Error saving request: {:?}", e);
            msg(Some(e))
        }
    })
}
