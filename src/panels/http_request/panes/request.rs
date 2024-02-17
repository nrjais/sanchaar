use iced::widget::{container, text};

use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum RequestMsg {}

impl RequestMsg {
    pub(crate) fn update(&self, state: &mut AppState) {

    }
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestMsg> {
    container(text("Request Pane"))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
