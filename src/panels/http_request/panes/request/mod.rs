mod body_editor;

use iced::widget::Column;
use iced::{widget::text, Length};

use crate::components::{CodeEditorMsg, ContentType};
use crate::state::request::RequestRawBody;
use crate::{
    components::{button_tab, button_tabs, key_value_editor, ButtonTabLabel, KeyValUpdateMsg},
    state::{request::ReqTabId, AppState},
};

#[derive(Debug, Clone)]
pub enum RequestPaneMsg {
    TabSelected(ReqTabId),
    Headers(KeyValUpdateMsg),
    Queries(KeyValUpdateMsg),
    BodyEditorAction(CodeEditorMsg),
    FormBodyEditAction(KeyValUpdateMsg),
}

impl RequestPaneMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        let request = &mut state.active_tab_mut().request;
        match self {
            RequestPaneMsg::TabSelected(tab) => {
                request.tab = tab;
            }
            RequestPaneMsg::Headers(msg) => {
                request.headers.update(msg);
            }
            RequestPaneMsg::Queries(msg) => {
                request.query_params.update(msg);
            }
            RequestPaneMsg::BodyEditorAction(action) => match &mut request.body {
                RequestRawBody::Json(content)
                | RequestRawBody::XML(content)
                | RequestRawBody::Text(content) => action.update(content),
                _ => {}
            },
            RequestPaneMsg::FormBodyEditAction(edit) => {
                if let RequestRawBody::Form(form) = &mut request.body {
                    form.update(edit);
                }
            }
        }
    }
}

fn body_tab(body: &RequestRawBody) -> iced::Element<RequestPaneMsg> {
    match body {
        RequestRawBody::Json(content) => body_editor::view(content, ContentType::Json),
        RequestRawBody::XML(content) => body_editor::view(content, ContentType::XML),
        RequestRawBody::Text(content) => body_editor::view(content, ContentType::Text),
        RequestRawBody::Form(values) => key_value_editor(values)
            .on_change(RequestPaneMsg::FormBodyEditAction)
            .element(),
        RequestRawBody::File(_) | RequestRawBody::None => text("No body").into(),
    }
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestPaneMsg> {
    let request = &state.active_tab().request;

    let tab_content = match request.tab {
        ReqTabId::Params => key_value_editor(&request.query_params)
            .on_change(RequestPaneMsg::Queries)
            .element(),
        ReqTabId::Headers => key_value_editor(&request.headers)
            .on_change(RequestPaneMsg::Headers)
            .element(),
        ReqTabId::Body => body_tab(&request.body),
    };

    let tabs = button_tabs(
        request.tab,
        &[
            button_tab(ReqTabId::Params, ButtonTabLabel::Text(text("Params"))),
            button_tab(ReqTabId::Headers, ButtonTabLabel::Text(text("Headers"))),
            button_tab(ReqTabId::Body, ButtonTabLabel::Text(text("Body"))),
        ],
        RequestPaneMsg::TabSelected,
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
