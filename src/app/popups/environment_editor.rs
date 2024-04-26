use std::borrow::Cow;

use components::{button_tab, button_tabs, vertical_button_tabs};
use iced::widget::{container, text};
use iced::{Command, Element, Length};

use crate::state::{AppState, TabKey};

#[derive(Debug, Clone)]
pub enum Message {
    Done,
}

impl Message {
    pub fn update(self, _state: &mut AppState) -> Command<Message> {
        Command::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Edit Environment")
}

pub fn done() -> Option<Message> {
    Some(Message::Done)
}

pub(crate) fn view(_state: &AppState, _tab: TabKey) -> Element<Message> {
    const envs: [&'static str; 3] = ["Dev", "Staging", "Production"];
    let tabs = envs.map(|tab| button_tab(tab, || text("hello")));
    container(vertical_button_tabs(
        "Dev",
        &tabs,
        |tab| Message::Done,
        None,
    ))
    .into()
}
