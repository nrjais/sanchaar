use iced::widget::container;

use crate::state::AppState;

pub mod http_request;

#[derive(Debug, Clone)]
pub enum PanelMsg {
    Http(http_request::HttpMsg),
}

impl PanelMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            PanelMsg::Http(msg) => msg.update(state),
        }
    }
}

pub fn view(state: &AppState) -> iced::Element<PanelMsg> {
    let req = http_request::view(state).map(PanelMsg::Http);

    container::Container::new(req)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .padding(4)
        .into()
}
