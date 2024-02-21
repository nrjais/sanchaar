use iced::Command;

use crate::{
    commands::CommandResultMsg,
    panels::{self, PanelMsg},
    state::AppState,
};

#[derive(Debug)]
pub enum AppMsg {
    Panel(PanelMsg),
    Command(CommandResultMsg),
}

impl AppMsg {
    pub fn update(self, state: &mut AppState) -> Command<AppMsg> {
        match self {
            AppMsg::Panel(msg) => msg.update(state),
            AppMsg::Command(msg) => return msg.update(state),
        }
        Command::none()
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    panels::view(state).map(AppMsg::Panel)
}
