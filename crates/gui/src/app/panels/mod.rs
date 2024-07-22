use iced::widget::container;
use iced::Task;

use crate::state::{AppState, Tab};

pub mod collection;
pub mod http;

#[derive(Debug, Clone)]
pub enum PanelMsg {
    Http(http::HttpTabMsg),
}

impl PanelMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            PanelMsg::Http(msg) => msg.update(state).map(PanelMsg::Http),
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a Tab) -> iced::Element<'a, PanelMsg> {
    let req = match tab {
        Tab::Http(tab) => http::view(state, tab).map(PanelMsg::Http),
        Tab::Collection(_) => todo!(),
    };

    container::Container::new(req)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
