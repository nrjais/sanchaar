mod collection_tree;
mod main_page;
mod panels;
mod popups;

use components::modal::modal;
use iced::Command;
use popups::PopupMsg;

use crate::app::main_page::MainPageMsg;
use crate::commands::commands;
use crate::{commands::CommandResultMsg, AppState};

#[derive(Debug)]
pub enum AppMsg {
    Command(CommandResultMsg),
    MainPage(MainPageMsg),
    Popup(PopupMsg),
}

impl AppMsg {
    pub fn update(self, state: &mut AppState) -> Command<AppMsg> {
        match self {
            AppMsg::Command(msg) => msg.update(state),
            AppMsg::MainPage(msg) => msg.update(state),
            AppMsg::Popup(msg) => msg.update(state),
        };
        commands(state)
    }
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    let main_page = main_page::view(state).map(AppMsg::MainPage);

    if let Some(ref popup) = state.popup {
        let popup = popups::view(state, popup).map(AppMsg::Popup);
        modal(main_page, popup).into()
    } else {
        main_page
    }
}
