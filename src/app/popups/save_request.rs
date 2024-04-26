use std::borrow::Cow;

use iced::widget::{horizontal_space, text, text_input, Column, Row};
use iced::{Command, Element};

use core::http::environment::EnvironmentKey;

use crate::state::popups::SaveRequestState;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    SelectEnv(EnvironmentKey),
}

impl Message {
    pub fn update(self, _state: &mut AppState) -> Command<Message> {
        Command::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Save Request")
}

pub fn done() -> Option<Message> {
    Some(Message::Done)
}

pub(crate) fn view<'a>(state: &'a AppState, data: &'a SaveRequestState) -> Element<'a, Message> {
    let name = Row::new()
        .push(text("Name"))
        .push(horizontal_space())
        .push(text_input("Name", &data.name))
        .spacing(4);

    let path = Row::new()
        .push(text("Location"))
        .push(horizontal_space())
        .align_items(iced::Alignment::Center)
        .spacing(4);

    Column::new().push(name).push(path).spacing(4).into()
}
