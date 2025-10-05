use iced::Task;
use iced_plugins::PluginMessage;

use crate::components::modal;
use popups::PopupMsg;

use crate::app::content_section::MainPageMsg;
use crate::{commands, hotkeys};
use crate::{commands::TaskMsg, state::AppState};

pub mod bottom_bar;
mod collection_tree;
mod content_section;
mod panels;
mod popups;

#[derive(Debug, Clone)]
pub enum AppMsg {
    Command(TaskMsg),
    MainPage(MainPageMsg),
    Popup(PopupMsg),
    Subscription(hotkeys::Message),
    Plugin(PluginMessage),
}

pub fn update(state: &mut AppState, msg: AppMsg) -> Task<AppMsg> {
    let cmd = match msg {
        AppMsg::Command(msg) => msg.update(state).map(AppMsg::Command),
        AppMsg::MainPage(msg) => msg.update(state).map(AppMsg::MainPage),
        AppMsg::Popup(msg) => msg.update(state).map(AppMsg::Popup),
        AppMsg::Subscription(msg) => msg.update(state).map(AppMsg::Subscription),
        AppMsg::Plugin(msg) => state.manager.update(msg).map(AppMsg::Plugin),
    };
    Task::batch([cmd, commands::background(state).map(AppMsg::Command)])
}

pub fn view(state: &AppState) -> iced::Element<AppMsg> {
    let main_page = content_section::view(state).map(AppMsg::MainPage);

    if let Some(ref popup) = state.common.popup {
        let popup = popups::view(state, popup).map(AppMsg::Popup);
        modal(main_page, popup, AppMsg::Popup(PopupMsg::Ignore))
    } else {
        // main_page.explain(components::colors::CYAN)
        main_page
    }
}
