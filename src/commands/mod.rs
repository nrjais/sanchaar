use std::mem;
use std::path::PathBuf;

use iced::widget::text_editor;
use iced::Command;
use serde_json::Value;

use cancellable_task::cancellable_task;

use crate::commands::cancellable_task::TaskResult;
use crate::core::persistence::collections;
use crate::core::persistence::fs::save_req_to_file;
use crate::core::persistence::request::{encode_request, read_request};
use crate::state::collection::{Collection, Item};
use crate::state::request::Request;
use crate::state::response::{CompletedResponse, ResponseState};
use crate::state::{CollectionKey, TabKey};
use crate::transformers::request::transform_request;
use crate::{app::AppMsg, core::client, state::AppState};

mod cancellable_task;

#[derive(Debug)]
pub enum AppCommand {
    InitRequest(TabKey),
    SendRequest(TabKey, reqwest::Request),
    SaveRequest(TabKey),
    OpenRequest(CollectionKey, Item),
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
    CollectionLoaded(Collection),
    Completed(&'static str),
    OpenRequestTab(CollectionKey, PathBuf, Request),
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
                state.commands.0.push(AppCommand::SendRequest(tab, req));
            }
            CommandResultMsg::CollectionLoaded(collection) => {
                state.collections.insert(collection);
            }
            CommandResultMsg::Completed(msg) => {
                println!("Command complete: {}", msg);
            }
            CommandResultMsg::OpenRequestTab(col, path, req) => {
                state.open_request(col, path, req);
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

pub fn commands(state: &mut AppState) -> Command<AppMsg> {
    let cmds = state.commands.take();
    if cmds.is_empty() {
        return Command::none();
    };
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
                let future = client::send_request(state.client.clone(), req);
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

                match sel_tab.req_ref.as_ref() {
                    Some((_, path)) => {
                        Command::perform(save_req_to_file(path.clone(), req), move |r| match r {
                            Ok(_) => CommandResultMsg::Completed("Request saved"),
                            Err(e) => {
                                println!("Error saving request: {:?}", e);
                                CommandResultMsg::Completed("Error saving request")
                            }
                        })
                    }
                    None => Command::none(),
                }
            }
            AppCommand::OpenRequest(col, item) => {
                let path = item.path.clone();
                Command::perform(read_request(path), move |r| match r {
                    Ok(req) => CommandResultMsg::OpenRequestTab(col, item.path.clone(), req),
                    Err(_) => {
                        println!("Error opening request: {:?}", item);
                        CommandResultMsg::Completed("Error opening request")
                    }
                })
            }
        };
        Some(cmd.map(AppMsg::Command))
    });

    Command::batch(cmds)
}

pub async fn load_collections() -> anyhow::Result<Collection> {
    let collection = match collections::load().await {
        Ok(c) => c,
        Err(e) => {
            println!("Error loading collection: {:?}", e);
            let collection = Collection::default();
            collections::save(&collection).await?;
            collection
        }
    };
    Ok(collection)
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections(), |r| match r {
        Ok(collection) => CommandResultMsg::CollectionLoaded(collection),
        Err(e) => {
            println!("Error init collection: {:?}", e);
            CommandResultMsg::CollectionLoaded(Collection::default())
        }
    })
    .map(AppMsg::Command)
}
