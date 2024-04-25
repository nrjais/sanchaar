use std::borrow::Cow;

use iced::widget::text;
use iced::{Command, Element};

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
    text("Edit Environment").into()
}
