use std::mem;

use iced::widget::text_editor;
use iced::Subscription;
use serde_json::Value;

use crate::transformers::request::transform_request;
use crate::{
    app::AppMsg,
    core::client,
    state::{request::Request, AppState},
};

#[derive(Debug, Clone)]
pub enum Command {
    SendRequest(Request),
}

#[derive(Debug, Clone)]
pub enum CommandMsg {
    UpdateResponse(client::Response),
}

fn pretty_body(body: &[u8]) -> String {
    let json = serde_json::from_slice::<Value>(body);
    if let Ok(json) = json {
        serde_json::to_string_pretty(&json).unwrap()
    } else {
        String::from_utf8_lossy(body).to_string()
    }
}

impl CommandMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            CommandMsg::UpdateResponse(res) => {
                let active_tab = state.active_tab_mut();
                let content = text_editor::Content::with_text(pretty_body(&res.body.data).as_str());
                active_tab.response.response = Some(res);
                active_tab.response.text_viewer = content;
            }
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn send_request(&mut self, req: Request) {
        self.0.push(Command::SendRequest(req));
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
        Command::SendRequest(req) => {
            let req = transform_request(&state.ctx.client, req).expect("Failed to create request");
            iced::Command::perform(client::send_request(req), |r| {
                AppMsg::Command(CommandMsg::UpdateResponse(
                    r.expect("Failed to send request"),
                ))
            })
        }
    });

    iced::Command::batch(cmds)
}

pub fn subscriptions() -> Subscription<AppMsg> {
    Subscription::none()
}
