mod collection_tree;
pub mod main_page;
pub mod panels;

use iced::Command;

use crate::app::main_page::MainPageMsg;
use crate::state::commands::commands;
use crate::state::{commands::CommandResultMsg, AppState};

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
