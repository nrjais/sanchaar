use iced::Command;

use crate::{
    panels::{self, PanelMsg},
    state::AppState,
};

#[derive(Debug, Clone)]
pub enum AppMsg {
    Panel(PanelMsg),
}

impl AppMsg {
    pub fn update(&self, state: &mut AppState) -> Command<AppMsg> {
        match self {
            AppMsg::Panel(msg) => msg.update(state),
        }
        Command::none()
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    panels::view(state).map(AppMsg::Panel)
}
