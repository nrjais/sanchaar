mod main_page;

use iced::Command;

use crate::app::main_page::MainPageMsg;
use crate::commands::commands;
use crate::{commands::CommandResultMsg, state::AppState};

#[derive(Debug)]
pub enum AppMsg {
    Command(CommandResultMsg),
    MainPage(MainPageMsg),
}

impl AppMsg {
    pub fn update(self, state: &mut AppState) -> Command<AppMsg> {
        match self {
            AppMsg::Command(msg) => msg.update(state),
            AppMsg::MainPage(msg) => msg.update(state),
        };
        commands(state)
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    main_page::view(state).map(AppMsg::MainPage)
}
