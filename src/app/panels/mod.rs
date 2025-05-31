use iced::widget::container;
use iced::Task;

use crate::state::{AppState, Tab};

pub mod collection;
pub mod cookie_store;
pub mod http;

#[derive(Debug, Clone)]
pub enum PanelMsg {
    HttpTab(http::HttpTabMsg),
    CollectionTab(collection::CollectionTabMsg),
    CookiesTab(cookie_store::CookieTabMsg),
}

impl PanelMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            PanelMsg::HttpTab(msg) => msg.update(state).map(PanelMsg::HttpTab),
            PanelMsg::CollectionTab(msg) => msg.update(state).map(PanelMsg::CollectionTab),
            PanelMsg::CookiesTab(msg) => msg.update(state).map(PanelMsg::CookiesTab),
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a Tab) -> iced::Element<'a, PanelMsg> {
    let req = match tab {
        Tab::Http(tab) => http::view(state, tab).map(PanelMsg::HttpTab),
        Tab::Collection(tab) => {
            let col = state.common.collections.get(tab.collection_key).unwrap();
            collection::view(tab, col).map(PanelMsg::CollectionTab)
        }
        Tab::CookieStore(tab) => cookie_store::view(tab).map(PanelMsg::CookiesTab),
    };

    container::Container::new(req)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
