use iced::{
    widget::{container, text},
    Length,
};

use crate::{
    components::{button_tab, button_tabs, keyval_editor, ButtonTabLabel, KeyValUpdateMsg},
    state::{request::ReqTabId, AppState},
};

#[derive(Debug, Clone)]
pub enum RequestMsg {
    TabSelected(ReqTabId),
    Headers(KeyValUpdateMsg),
    Queries(KeyValUpdateMsg),
}

impl RequestMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            RequestMsg::TabSelected(tab) => {
                state.active_tab_mut().request.tab = tab;
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

    let tabs = button_tabs(
        request.tab,
        Vec::from([
            button_tab(
                ReqTabId::Params,
                ButtonTabLabel::Text(text("Params")),
                keyval_editor(&request.query_params)
                    .on_change(RequestMsg::Queries)
                    .element(),
            ),
            button_tab(
                ReqTabId::Headers,
                ButtonTabLabel::Text(text("Headers")),
                keyval_editor(&request.headers)
                    .on_change(RequestMsg::Headers)
                    .element(),
            ),
            button_tab(
                ReqTabId::Body,
                ButtonTabLabel::Text(text("Body")),
                text::Text::new("Body").into(),
            ),
        ]),
        RequestMsg::TabSelected,
    );

    container(tabs)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
