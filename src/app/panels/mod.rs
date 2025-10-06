use iced::Task;
use iced::widget::container;

use crate::state::{AppState, Tab};

pub mod collection;
pub mod cookie_store;
pub mod history;
pub mod http;
pub mod perf;

#[derive(Debug, Clone)]
pub enum PanelMsg {
    Http(http::HttpTabMsg),
    Collection(collection::CollectionTabMsg),
    Cookies(cookie_store::CookieTabMsg),
    History(history::HistoryTabMsg),
    Perf(perf::PerfTabMsg),
}

impl PanelMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            PanelMsg::Http(msg) => msg.update(state).map(PanelMsg::Http),
            PanelMsg::Collection(msg) => msg.update(state).map(PanelMsg::Collection),
            PanelMsg::Cookies(msg) => msg.update(state).map(PanelMsg::Cookies),
            PanelMsg::History(msg) => msg.update(state).map(PanelMsg::History),
            PanelMsg::Perf(msg) => msg.update(state).map(PanelMsg::Perf),
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a Tab) -> iced::Element<'a, PanelMsg> {
    let req = match tab {
        Tab::Http(tab) => http::view(state, tab).map(PanelMsg::Http),
        Tab::Collection(tab) => {
            let col = state.common.collections.get(tab.collection_key).unwrap();
            collection::view(tab, col).map(PanelMsg::Collection)
        }
        Tab::CookieStore(tab) => cookie_store::view(tab).map(PanelMsg::Cookies),
        Tab::History(tab) => history::view(state, tab).map(PanelMsg::History),
        Tab::Perf(tab) => perf::view(state, tab).map(PanelMsg::Perf),
    };

    container::Container::new(req)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
