use std::mem;

use iced::Subscription;

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

impl CommandMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            CommandMsg::UpdateResponse(response) => {
                state.active_tab_mut().response = Some(response);
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
