mod cancellable_task;

use std::mem;

use iced::widget::text_editor;

use cancellable_task::cancellable_task;
use serde_json::Value;

use crate::state::response::{CompletedResponse, ResponseState};

use crate::commands::cancellable_task::TaskResult;
use crate::state::TabKey;
use crate::transformers::request::transform_request;
use crate::{app::AppMsg, core::client, state::AppState};

#[derive(Debug)]
pub enum Command {
    InitRequest(TabKey),
    SendRequest(TabKey, reqwest::Request),
}

#[derive(Debug)]
pub enum ResponseResult {
    Completed(client::Response),
    Error(anyhow::Error),
    Cancelled,
}

#[derive(Debug)]
pub enum CommandResultMsg {
    UpdateResponse(TabKey, ResponseResult),
    RequestReady(TabKey, reqwest::Request),
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
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Completed(res)) => {
                state.cancel_tab_tasks(tab);
                let active_tab = state.active_tab_mut();
                let content = text_editor::Content::with_text(pretty_body(&res.body.data).as_str());
                active_tab.response.state = ResponseState::Completed(CompletedResponse {
                    result: res,
                    content,
                });
            }
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Error(e)) => {
                state.cancel_tab_tasks(tab);
                let active_tab = state.active_tab_mut();
                active_tab.response.state = ResponseState::Failed(e);
            }
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Cancelled) => {
                // Response state is already updated to idle in cancel_tasks
                state.clear_tab_tasks(tab);
            }
            CommandResultMsg::RequestReady(tab, req) => {
                state.cancel_tab_tasks(tab);
                state.commands.0.push(Command::SendRequest(tab, req));
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

    pub fn send_request(&mut self, tab: TabKey) {
        self.0.push(Command::InitRequest(tab));
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
    let cmds = cmds.into_iter().filter_map(|cmd| {
        let cmd = match cmd {
            Command::InitRequest(tab) => {
                let client = state.ctx.client.clone();
                let sel_tab = state.get_tab_mut(tab)?;
                let req = transform_request(client, sel_tab.request.to_request());
                let (cancel_tx, req) = cancellable_task(req);

                sel_tab.add_task(cancel_tx);
                sel_tab.response.state = ResponseState::Executing;

                iced::Command::perform(req, move |r| match r {
                    TaskResult::Completed(req) => match req {
                        Ok(req) => CommandResultMsg::RequestReady(tab, req),
                        Err(e) => CommandResultMsg::UpdateResponse(tab, ResponseResult::Error(e)),
                    },
                    TaskResult::Cancelled => {
                        CommandResultMsg::UpdateResponse(tab, ResponseResult::Cancelled)
                    }
                })
                .map(AppMsg::Command)
            }

            Command::SendRequest(tab, req) => {
                let sel_tab = state.get_tab_mut(tab)?;
                let (cancel_tx, req) = cancellable_task(client::send_request(req));
                sel_tab.add_task(cancel_tx);
                sel_tab.response.state = ResponseState::Executing;

                iced::Command::perform(req, move |r| match r {
                    TaskResult::Completed(Ok(res)) => {
                        CommandResultMsg::UpdateResponse(tab, ResponseResult::Completed(res))
                    }
                    TaskResult::Completed(Err(e)) => {
                        CommandResultMsg::UpdateResponse(tab, ResponseResult::Error(e))
                    }
                    TaskResult::Cancelled => {
                        CommandResultMsg::UpdateResponse(tab, ResponseResult::Cancelled)
                    }
                })
                .map(AppMsg::Command)
            }
        };
        Some(cmd)
    });

    iced::Command::batch(cmds)
}
