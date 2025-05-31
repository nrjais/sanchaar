pub mod env_editor;
mod settings;

use core::http::collection::Collection;

use components::{button_tab, button_tabs};
use iced::widget::{text, Column};
use iced::Length;
use iced::{Element, Task};

use crate::state::tabs::collection_tab::{CollectionTab, CollectionTabId};
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum CollectionTabMsg {
    TabChange(CollectionTabId),
    EnvEditor(env_editor::Message),
    Settings(settings::Message),
}

impl CollectionTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Collection(tab)) = state.active_tab_mut() else {
            return Task::none();
        };
        match self {
            CollectionTabMsg::TabChange(id) => {
                tab.tab = id;
                Task::none()
            }
            CollectionTabMsg::EnvEditor(msg) => msg.update(state).map(CollectionTabMsg::EnvEditor),
            CollectionTabMsg::Settings(msg) => msg.update(state).map(CollectionTabMsg::Settings),
        }
    }
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, CollectionTabMsg> {
    let tab_content = match tab.tab {
        CollectionTabId::Environments => {
            env_editor::view(tab, col).map(CollectionTabMsg::EnvEditor)
        }
        CollectionTabId::Settings => settings::view(tab, col).map(CollectionTabMsg::Settings),
    };

    let tabs = button_tabs(
        tab.tab,
        [
            button_tab(CollectionTabId::Settings, || text("Settings")),
            button_tab(CollectionTabId::Environments, || text("Environments")),
        ]
        .into_iter(),
        CollectionTabMsg::TabChange,
        None,
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(4)
        .into()
}
