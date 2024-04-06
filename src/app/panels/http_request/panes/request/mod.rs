use iced::widget::{container, horizontal_space, pick_list, Column, Row};
use iced::{widget::text, Length};

use crate::components::{icon, icons, CodeEditorMsg, ContentType, KeyValList};
use crate::state::request::{RequestPane, RequestRawBody};
use crate::{
    components::{button_tab, button_tabs, key_value_editor, KeyValUpdateMsg},
    state::{request::ReqTabId, AppState},
};

mod body_editor;

#[derive(Debug, Clone)]
pub enum RequestPaneMsg {
    TabSelected(ReqTabId),
    Headers(KeyValUpdateMsg),
    Queries(KeyValUpdateMsg),
    PathParams(KeyValUpdateMsg),
    BodyEditorAction(CodeEditorMsg),
    FormBodyEditAction(KeyValUpdateMsg),
    ChangeBodyType(&'static str),
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
            RequestPaneMsg::PathParams(msg) => {
                request.path_params.update(msg);
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
            RequestPaneMsg::ChangeBodyType(content_type) => {
                request.body = match content_type {
                    "URL Encoded" => RequestRawBody::Form(KeyValList::new()),
                    "Json" => RequestRawBody::Json(Default::default()),
                    "XML" => RequestRawBody::XML(Default::default()),
                    "Text" => RequestRawBody::Text(Default::default()),
                    "File" => RequestRawBody::File(Default::default()),
                    "None" => RequestRawBody::None,
                    _ => RequestRawBody::None,
                };
            }
        }
    }
}

fn body_tab(body: &RequestRawBody) -> iced::Element<RequestPaneMsg> {
    let size = 14;
    let header = Row::new()
        .push(text(format!("Content Type: {}", body.as_str())).size(size))
        .push(horizontal_space())
        .push(
            pick_list(
                RequestRawBody::all_variants(),
                Some(body.as_str()),
                RequestPaneMsg::ChangeBodyType,
            )
            .text_size(size)
            .padding([2, 4]),
        )
        .height(Length::Shrink)
        .align_items(iced::Alignment::Center);

    let body = match body {
        RequestRawBody::Json(content) => body_editor::view(content, ContentType::Json),
        RequestRawBody::XML(content) => body_editor::view(content, ContentType::XML),
        RequestRawBody::Text(content) => body_editor::view(content, ContentType::Text),
        RequestRawBody::Form(values) => container(
            key_value_editor(values)
                .on_change(RequestPaneMsg::FormBodyEditAction)
                .element(),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into(),
        RequestRawBody::File(_) | RequestRawBody::None => {
            let empty_icon = container(icon(icons::FileCancel).size(60.0))
                .padding(10)
                .center_x()
                .width(Length::Fill)
                .height(Length::Shrink);

            container(
                Column::new()
                    .push(empty_icon)
                    .push(text("No Body Content"))
                    .height(Length::Shrink)
                    .width(Length::Shrink),
            )
            .into()
        }
    };

    Column::new()
        .push(header)
        .push(
            container(body)
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y(),
        )
        .spacing(4)
        .into()
}

fn params_view(request: &RequestPane) -> iced::Element<RequestPaneMsg> {
    let query = key_value_editor(&request.query_params)
        .on_change(RequestPaneMsg::Queries)
        .element();

    let has_params = request.path_params.size() > 0;
    let path = has_params.then(|| {
        Column::new()
            .push("Path Params")
            .push(
                key_value_editor(&request.path_params)
                    .on_change(RequestPaneMsg::PathParams)
                    .element(),
            )
            .spacing(4)
    });

    Column::new()
        .push(Column::new().push("Query Params").push(query).spacing(4))
        .push_maybe(path)
        .spacing(8)
        .into()
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestPaneMsg> {
    let request = &state.active_tab().request;

    let tab_content = match request.tab {
        ReqTabId::Params => params_view(&request),
        ReqTabId::Headers => key_value_editor(&request.headers)
            .on_change(RequestPaneMsg::Headers)
            .element(),
        ReqTabId::Body => body_tab(&request.body),
    };

    let tabs = button_tabs(
        request.tab,
        &[
            button_tab(ReqTabId::Params, || text("Params")),
            button_tab(ReqTabId::Headers, || text("Headers")),
            button_tab(ReqTabId::Body, || text("Body")),
        ],
        RequestPaneMsg::TabSelected,
        None,
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(4)
        .into()
}
