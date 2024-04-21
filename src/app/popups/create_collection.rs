use crate::state::AppState;
use iced::widget::text;
use iced::Element;

#[derive(Clone, Debug)]
pub enum Message {}

impl Message {
    pub fn update(self, _state: &mut AppState) {
        {}
    }
}

pub(crate) fn view(_state: &AppState) -> Element<Message> {
    text("Create Collection").into()
}
