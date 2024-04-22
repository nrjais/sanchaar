use crate::state::AppState;
use iced::widget::{text, Column};
use iced::Element;
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub enum Message {
    Done,
}

impl Message {
    pub fn update(self, _state: &mut AppState) {
        {}
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Create Collection")
}

pub(crate) fn view(_state: &AppState) -> Element<Message> {
    Column::new()
        .push(text("Collection Name:"))
        .push(text("Collection Description:"))
        .spacing(4)
        .into()
}
