use std::mem;
use std::sync::Arc;

use iced::Command;
use serde_json::Value;
use tokio::fs;

use components::text_editor;
use core::client;
use core::http::collection::Collection;
use core::http::request::Request;
use core::http::CollectionRequest;
use core::persistence::collections;
use core::persistence::request::save_req_to_file;
use core::persistence::request::{encode_request, read_request};
use text_editor::Content;

use crate::commands::builders::send_request_cmd;
use crate::state::response::{BodyMode, CompletedResponse, ResponseState};
use crate::state::TabKey;
use crate::{app::AppMsg, AppState};

mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug)]
pub enum AppCommand {
    SendRequest(TabKey),
    SaveRequest(TabKey),
    OpenRequest(CollectionRequest),
    RenameRequest(CollectionRequest, String),
}

#[derive(Debug, Clone)]
pub enum ResponseResult {
    Completed(client::Response),
    Error(Arc<anyhow::Error>),
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum CommandResultMsg {
    UpdateResponse(TabKey, ResponseResult),
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
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CommandResultMsg::UpdateResponse(tab, ResponseResult::Completed(res)) => {
                state.cancel_tab_tasks(tab);
                let Some(tab_mut) = state.get_tab_mut(tab) else {
                    return Command::none();
                };

                let (raw, pretty) = pretty_body(&res.body.data);
                tab_mut.response.state = ResponseState::Completed(CompletedResponse {
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
        Command::none()
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
            AppCommand::SendRequest(tab) => send_request_cmd(state, tab),
            AppCommand::SaveRequest(tab) => {
                let sel_tab = state.get_tab(tab)?;
                let req_ref = state.col_req_ref(tab)?;

                let req = sel_tab.request.to_request();
                let encoded = encode_request(&req);
                Command::perform(
                    save_req_to_file(req_ref.path.clone(), encoded),
                    move |r| match r {
                        Ok(_) => CommandResultMsg::Completed("Request saved"),
                        Err(e) => {
                            println!("Error saving request: {:?}", e);
                            CommandResultMsg::Completed("Error saving request")
                        }
                    },
                )
            }
            AppCommand::OpenRequest(col) => {
                let req = state.collections.get_ref(&col)?;
                Command::perform(read_request(req.path.clone()), move |r| match r {
                    Ok(req) => CommandResultMsg::OpenRequestTab(col, req),
                    Err(_) => {
                        println!("Error opening request: {:?}", col);
                        CommandResultMsg::Completed("Error opening request")
                    }
                })
            }
            AppCommand::RenameRequest(req, new) => {
                let (old, new) = state.collections.rename_request(req, new)?;
                Command::perform(fs::rename(old, new), move |_| {
                    CommandResultMsg::Completed("Request renamed")
                })
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
        println!("Error loading http: {:?}", e);
        vec![]
    })
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections(), CommandResultMsg::CollectionsLoaded).map(AppMsg::Command)
}
