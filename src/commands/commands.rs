use std::sync::Arc;

use iced::futures::TryFutureExt;
use iced::Command;

use core::client::send_request;
use core::transformers::request::transform_request;

use crate::commands::cancellable_task::{cancellable_task, TaskResult};
use crate::commands::{CommandResultMsg, ResponseResult};
use crate::state::response::ResponseState;
use crate::state::{AppState, TabKey};

pub fn send_request_cmd(state: &mut AppState, tab: TabKey) -> Command<CommandResultMsg> {
    let client = state.client.clone();
    let Some(sel_tab) = state.get_tab_mut(tab) else {
        return Command::none();
    };
    sel_tab.response.state = ResponseState::Executing;

    let request = sel_tab.request.to_request();

    let req_fut =
        transform_request(client.clone(), request).and_then(|req| send_request(client, req));

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
