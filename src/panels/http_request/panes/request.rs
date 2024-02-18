use iced::widget::{container, text};
use iced_aw::{TabLabel, Tabs};

use crate::{
    components::{keyval_editor, KeyValUpdateMsg},
    state::{request::ReqTabId, AppState},
};

#[derive(Debug, Clone)]
pub enum RequestMsg {
    TabSelected(ReqTabId),
    Headers(KeyValUpdateMsg),
    Queries(KeyValUpdateMsg),
}

impl RequestMsg {
    pub(crate) fn update(&self, state: &mut AppState) {
        match self {
            RequestMsg::TabSelected(tab) => {
                state.active_tab_mut().request.tab = *tab;
            }
            RequestMsg::Headers(msg) => {
                state.active_tab_mut().request.headers.update(msg);
            }
            RequestMsg::Queries(msg) => {
                state.active_tab_mut().request.query_params.update(msg);
            }
        }
    }
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestMsg> {
    let request = &state.active_tab().request;

    let tabs = Tabs::new(RequestMsg::TabSelected)
        .push(
            ReqTabId::Queries,
            TabLabel::Text("Queries".into()),
            keyval_editor(&request.query_params)
                .on_change(RequestMsg::Queries)
                .element(),
        )
        .push(ReqTabId::Body, TabLabel::Text("Body".into()), text("Body"))
        .push(
            ReqTabId::Headers,
            TabLabel::Text("Headers".into()),
            keyval_editor(&request.headers)
                .on_change(RequestMsg::Headers)
                .element(),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .set_active_tab(&request.tab);

    container(tabs)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
