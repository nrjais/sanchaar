pub mod env_editor;

use components::{button_tab, button_tabs, colors};
use iced::widget::{text, Column};
use iced::Length;
use iced::{widget::container, Element, Task};

use crate::state::collection_tab::{CollectionTab, CollectionTabId};
use crate::state::{AppState, Tab};


#[derive(Debug, Clone)]
pub enum CollectionTabMsg {
    TabChange(CollectionTabId),
    EnvEditor(env_editor::Message),
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
        }
    }
}

pub fn view<'a>(tab: &'a CollectionTab) -> Element<'a, CollectionTabMsg> {
    let tab_content = match tab.tab {
        CollectionTabId::Environments => env_editor::view(tab)
            .map(CollectionTabMsg::EnvEditor)
            .explain(colors::CYAN),
        CollectionTabId::Settings => todo!(),
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
