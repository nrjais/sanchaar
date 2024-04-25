use iced::widget::{container, horizontal_space, pick_list, Column, Row};
use iced::{widget::text, Command, Length};

use crate::state::request::{RawRequestBody, RequestPane};
use crate::state::{request::ReqTabId, AppState};
use components::{button_tab, button_tabs, key_value_editor, KeyValUpdateMsg};
use components::{icon, icons, CodeEditorMsg, ContentType};

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
    pub(crate) fn update(self, state: &mut AppState) -> Command<Self> {
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
                RawRequestBody::Json(content)
                | RawRequestBody::XML(content)
                | RawRequestBody::Text(content) => action.update(content),
                _ => {}
            },
            RequestPaneMsg::FormBodyEditAction(edit) => {
                if let RawRequestBody::Form(form) = &mut request.body {
                    form.update(edit);
                }
            }
            RequestPaneMsg::ChangeBodyType(content_type) => request.change_body_type(content_type),
        };
        Command::none()
    }
}

fn body_tab(body: &RawRequestBody) -> iced::Element<RequestPaneMsg> {
    let size = 14;
    let header = Row::new()
        .push(text(format!("Content Type: {}", body.as_str())).size(size))
        .push(horizontal_space())
        .push(
            pick_list(
                RawRequestBody::all_variants(),
                Some(body.as_str()),
                RequestPaneMsg::ChangeBodyType,
            )
            .text_size(size)
            .padding([2, 4]),
        )
        .height(Length::Shrink)
        .align_items(iced::Alignment::Center);

    let body = match body {
        RawRequestBody::Json(content) => body_editor::view(content, ContentType::Json),
        RawRequestBody::XML(content) => body_editor::view(content, ContentType::XML),
        RawRequestBody::Text(content) => body_editor::view(content, ContentType::Text),
        RawRequestBody::Form(values) => container(
            key_value_editor(values)
                .on_change(RequestPaneMsg::FormBodyEditAction)
                .element(),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into(),
        RawRequestBody::File(_) | RawRequestBody::None => {
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
        ReqTabId::Params => params_view(request),
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
