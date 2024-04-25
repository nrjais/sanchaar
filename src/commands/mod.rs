use std::mem;

use iced::Command;
use serde_json::Value;
use tokio::fs;

use cancellable_task::cancellable_task;
use cancellable_task::TaskResult;
use components::text_editor;
use core::client;
use core::client::send_request;
use core::collection::collection::Collection;
use core::collection::request::Request;
use core::collection::CollectionRequest;
use core::persistence::collections;
use core::persistence::fs::save_req_to_file;
use core::persistence::request::{encode_request, read_request};
use core::transformers::request::transform_request;
use text_editor::Content;

use crate::state::response::{BodyMode, CompletedResponse, ResponseState};
use crate::state::TabKey;
use crate::{app::AppMsg, AppState};

mod cancellable_task;
pub mod dialog;

#[derive(Debug)]
pub enum AppCommand {
    InitRequest(TabKey),
    SendRequest(TabKey, reqwest::Request),
    SaveRequest(TabKey),
    OpenRequest(CollectionRequest),
    RenameRequest(CollectionRequest, String),
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
    CollectionsLoaded(Vec<Collection>),
    Completed(&'static str),
    OpenRequestTab(CollectionRequest, Request),
}

fn pretty_body(body: &[u8]) -> (String, Option<String>) {
    let raw = String::from_utf8_lossy(body).to_string();

    let json = serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|v| serde_json::to_string_pretty(&v).ok());

    (raw, json)
}

impl CommandResultMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Completed(res)) => {
                state.cancel_tab_tasks(tab);
                let active_tab = state.active_tab_mut();
                let (raw, pretty) = pretty_body(&res.body.data);
                active_tab.response.state = ResponseState::Completed(CompletedResponse {
                    result: res,
                    content: pretty.map(|p| Content::with_text(p.as_str())),
                    raw: Content::with_text(raw.as_str()),
                    mode: BodyMode::Pretty,
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
                state.commands.0.push(AppCommand::SendRequest(tab, req));
            }
            CommandResultMsg::CollectionsLoaded(collection) => {
                state.collections.insert(collection);
            }
            CommandResultMsg::Completed(msg) => {
                println!("Command complete: {}", msg);
            }
            CommandResultMsg::OpenRequestTab(col, req) => {
                state.open_request(col, req);
            }
        };
    }
}

#[derive(Debug)]
pub struct Commands(Vec<AppCommand>);

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn take(&mut self) -> Vec<AppCommand> {
        mem::take(&mut self.0)
    }

    pub fn add(&mut self, cmd: AppCommand) {
        self.0.push(cmd);
    }
}

fn commands_inner(state: &mut AppState) -> Vec<Command<AppMsg>> {
    let cmds = state.commands.take();
    let cmds = cmds.into_iter().filter_map(|cmd| {
        let cmd = match cmd {
            AppCommand::InitRequest(tab) => {
                let client = state.client.clone();
                let sel_tab = state.get_tab_mut(tab)?;
                let req = transform_request(client, sel_tab.request.to_request());
                let (cancel_tx, req) = cancellable_task(req);

                sel_tab.add_task(cancel_tx);
                sel_tab.response.state = ResponseState::Executing;

                Command::perform(req, move |r| match r {
                    TaskResult::Completed(req) => match req {
                        Ok(req) => CommandResultMsg::RequestReady(tab, req),
                        Err(e) => CommandResultMsg::UpdateResponse(tab, ResponseResult::Error(e)),
                    },
                    TaskResult::Cancelled => {
                        CommandResultMsg::UpdateResponse(tab, ResponseResult::Cancelled)
                    }
                })
            }

            AppCommand::SendRequest(tab, req) => {
                let future = send_request(state.client.clone(), req);
                let (cancel_tx, req) = cancellable_task(future);
                let sel_tab = state.get_tab_mut(tab)?;
                sel_tab.add_task(cancel_tx);
                sel_tab.response.state = ResponseState::Executing;

                Command::perform(req, move |r| match r {
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
            }

            AppCommand::SaveRequest(tab) => {
                let sel_tab = state.get_tab(tab)?;
                let req = sel_tab.request.to_request();
                let req = encode_request(&req);

                match state.col_req_ref(tab) {
                    Some(req_ref) => {
                        Command::perform(save_req_to_file(req_ref.path.clone(), req), move |r| {
                            match r {
                                Ok(_) => CommandResultMsg::Completed("Request saved"),
                                Err(e) => {
                                    println!("Error saving request: {:?}", e);
                                    CommandResultMsg::Completed("Error saving request")
                                }
                            }
                        })
                    }
                    None => Command::none(),
                }
            }
            AppCommand::OpenRequest(col) => {
                if let Some(req) = state.collections.get_ref(&col) {
                    Command::perform(read_request(req.path.clone()), move |r| match r {
                        Ok(req) => CommandResultMsg::OpenRequestTab(col, req),
                        Err(_) => {
                            println!("Error opening request: {:?}", col);
                            CommandResultMsg::Completed("Error opening request")
                        }
                    })
                } else {
                    Command::none()
                }
            }
            AppCommand::RenameRequest(req, new) => {
                if let Some((old, new)) = state.collections.rename_request(req, new) {
                    Command::perform(fs::rename(old, new), move |_| {
                        CommandResultMsg::Completed("Request renamed")
                    })
                } else {
                    Command::none()
                }
            }
        };
        Some(cmd.map(AppMsg::Command))
    });
    cmds.collect()
}
pub fn commands(state: &mut AppState) -> Command<AppMsg> {
    let cmds = commands_inner(state);

    Command::batch(cmds)
}

pub fn commands_merged(state: &mut AppState, cmd: Command<AppMsg>) -> Command<AppMsg> {
    let mut commands = commands_inner(state);
    commands.push(cmd);
    Command::batch(commands)
}

pub async fn load_collections() -> Vec<Collection> {
    collections::load().await.unwrap_or_else(|e| {
        println!("Error loading collection: {:?}", e);
        vec![]
    })
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections(), CommandResultMsg::CollectionsLoaded).map(AppMsg::Command)
}
