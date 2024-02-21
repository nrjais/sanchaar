use iced::widget::Column;
use iced::{widget::text, Length};

use crate::{
    components::{button_tab, button_tabs, key_value_editor, ButtonTabLabel, KeyValUpdateMsg},
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

    let tab_content = match request.tab {
        ReqTabId::Params => key_value_editor(&request.query_params)
            .on_change(RequestMsg::Queries)
            .element(),
        ReqTabId::Body => text::Text::new("Body").into(),
        ReqTabId::Headers => key_value_editor(&request.headers)
            .on_change(RequestMsg::Headers)
            .element(),
    };

    let tabs = button_tabs(
        request.tab,
        &[
            button_tab(ReqTabId::Params, ButtonTabLabel::Text(text("Params"))),
            button_tab(ReqTabId::Headers, ButtonTabLabel::Text(text("Headers"))),
            button_tab(ReqTabId::Body, ButtonTabLabel::Text(text("Body"))),
        ],
        RequestMsg::TabSelected,
        None,
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(2)
        .into()
}
