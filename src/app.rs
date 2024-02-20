use iced::Command;

use crate::{
    commands::{self, CommandMsg},
    panels::{self, PanelMsg},
    state::AppState,
};

#[derive(Debug, Clone)]
pub enum AppMsg {
    Panel(PanelMsg),
    Command(CommandMsg),
}

impl AppMsg {
    pub fn update(self, state: &mut AppState) -> Command<AppMsg> {
        match self {
            AppMsg::Panel(msg) => msg.update(state),
            AppMsg::Command(msg) => msg.update(state),
        }
        commands::commands(state)
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    panels::view(state).map(AppMsg::Panel)
}
