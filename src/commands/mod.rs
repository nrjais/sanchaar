mod cancellable_task;

use std::mem;

use iced::widget::text_editor;

use cancellable_task::cancellable_task;
use serde_json::Value;

use crate::state::response::{CompletedResponse, ResponseState};

use crate::commands::cancellable_task::TaskResult;
use crate::state::TaskCancelKey;
use crate::transformers::request::transform_request;
use crate::{app::AppMsg, core::client, state::AppState};

#[derive(Debug)]
pub enum Command {
    InitRequest,
    SendRequest(reqwest::Request),
}

#[derive(Debug)]
pub enum ResponseResult {
    Completed(client::Response),
    Error(anyhow::Error),
    Cancelled,
}

#[derive(Debug)]
pub enum CommandResultMsg {
    UpdateResponse(ResponseResult, TaskCancelKey),
    RequestReady(reqwest::Request, TaskCancelKey),
}

fn pretty_body(body: &[u8]) -> String {
    let json = serde_json::from_slice::<Value>(body);
    if let Ok(json) = json {
        serde_json::to_string_pretty(&json).unwrap()
    } else {
        String::from_utf8_lossy(body).to_string()
    }
}

impl CommandResultMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            CommandResultMsg::UpdateResponse(ResponseResult::Completed(res), cancel) => {
                state.ctx.task_cancel_tx.remove(cancel);
                let active_tab = state.active_tab_mut();
                let content = text_editor::Content::with_text(pretty_body(&res.body.data).as_str());
                active_tab.response.state = ResponseState::Completed(CompletedResponse {
                    result: res,
                    content,
                });
            }
            CommandResultMsg::UpdateResponse(ResponseResult::Error(e), cancel) => {
                state.ctx.task_cancel_tx.remove(cancel);
                let active_tab = state.active_tab_mut();
                active_tab.response.state = ResponseState::Failed(e);
            }
            CommandResultMsg::UpdateResponse(ResponseResult::Cancelled, cancel) => {
                state.ctx.task_cancel_tx.remove(cancel);
                let active_tab = state.active_tab_mut();
                active_tab.response.state = ResponseState::Idle;
            }
            CommandResultMsg::RequestReady(req, cancel) => {
                state.ctx.task_cancel_tx.remove(cancel);
                state.commands.0.push(Command::SendRequest(req));
            }
        };
    }
}

#[derive(Debug)]
pub struct Commands(Vec<Command>);

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn send_request(&mut self) {
        self.0.push(Command::InitRequest);
    }

    pub fn take(&mut self) -> Vec<Command> {
        mem::take(&mut self.0)
    }
}

pub fn commands(state: &mut AppState) -> iced::Command<AppMsg> {
    let cmds = state.commands.take();
    if cmds.is_empty() {
        return iced::Command::none();
    };
    let cmds = cmds.into_iter().map(|cmd| match cmd {
        Command::InitRequest => {
            let active_tab = state.active_tab();

            let req = transform_request(state.ctx.client.clone(), active_tab.request.to_request());
            let (cancel_tx, req) = cancellable_task(req);

            let cancel_key = state.ctx.task_cancel_tx.insert(cancel_tx);
            state.active_tab_mut().response.state = ResponseState::Executing(cancel_key);

            iced::Command::perform(req, move |r| match r {
                TaskResult::Completed(req) => match req {
                    Ok(req) => CommandResultMsg::RequestReady(req, cancel_key),
                    Err(e) => {
                        CommandResultMsg::UpdateResponse(ResponseResult::Error(e), cancel_key)
                    }
                },
                TaskResult::Cancelled => {
                    CommandResultMsg::UpdateResponse(ResponseResult::Cancelled, cancel_key)
                }
            })
            .map(AppMsg::Command)
        }

        Command::SendRequest(req) => {
            let (cancel_tx, req) = cancellable_task(client::send_request(req));
            let cancel_key = state.ctx.task_cancel_tx.insert(cancel_tx);
            state.active_tab_mut().response.state = ResponseState::Executing(cancel_key);

            iced::Command::perform(req, move |r| match r {
                TaskResult::Completed(Ok(res)) => {
                    CommandResultMsg::UpdateResponse(ResponseResult::Completed(res), cancel_key)
                }
                TaskResult::Completed(Err(e)) => {
                    CommandResultMsg::UpdateResponse(ResponseResult::Error(e), cancel_key)
                }
                TaskResult::Cancelled => {
                    CommandResultMsg::UpdateResponse(ResponseResult::Cancelled, cancel_key)
                }
            })
            .map(AppMsg::Command)
        }
    });

    iced::Command::batch(cmds)
}
