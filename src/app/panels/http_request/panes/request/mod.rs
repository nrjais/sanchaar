use std::path::PathBuf;

use iced::widget::Column;
use iced::{widget::text, Command, Length};

use crate::commands::dialog::open_file_dialog;
use crate::state::request::{RawRequestBody, RequestPane};
use crate::state::{request::ReqTabId, AppState};
use components::{button_tab, button_tabs, key_value_editor, KeyValUpdateMsg};
use components::{CodeEditorMsg, FilePickerUpdateMsg};

use self::auth_editor::{auth_view, AuthEditorMsg};
use self::body_view::body_tab;

mod auth_editor;
mod body_editor;
mod body_view;

#[derive(Debug, Clone)]
pub enum RequestPaneMsg {
    TabSelected(ReqTabId),
    Headers(KeyValUpdateMsg),
    Queries(KeyValUpdateMsg),
    PathParams(KeyValUpdateMsg),
    BodyEditorAction(CodeEditorMsg),
    AuthEditorAction(AuthEditorMsg),
    FormBodyEditAction(KeyValUpdateMsg),
    MultipartParamsAction(KeyValUpdateMsg),
    MultipartFilesAction(FilePickerUpdateMsg),
    ChangeBodyFile(Option<PathBuf>),
    ChangeBodyType(&'static str),
    OpenFilePicker,
}

impl RequestPaneMsg {
    pub(crate) fn update(self, state: &mut AppState) -> Command<Self> {
        let request = state.active_tab_mut().request_mut();
        match self {
            Self::TabSelected(tab) => {
                request.tab = tab;
            }
            Self::Headers(msg) => {
                request.headers.update(msg);
            }
            Self::Queries(msg) => {
                request.query_params.update(msg);
            }
            Self::PathParams(msg) => {
                request.path_params.update(msg);
            }
            Self::BodyEditorAction(action) => match &mut request.body {
                RawRequestBody::Json(content)
                | RawRequestBody::XML(content)
                | RawRequestBody::Text(content) => action.update(content),
                _ => {}
            },
            Self::FormBodyEditAction(edit) => {
                if let RawRequestBody::Form(form) = &mut request.body {
                    form.update(edit);
                }
            }
            Self::MultipartParamsAction(action) => {
                if let RawRequestBody::Multipart(params, _) = &mut request.body {
                    params.update(action);
                }
            }
            Self::MultipartFilesAction(action) => {
                if let RawRequestBody::Multipart(_, files) = &mut request.body {
                    files.update(action);
                }
            }
            Self::ChangeBodyFile(path) => {
                request.body = RawRequestBody::File(path);
            }
            Self::ChangeBodyType(ct) => request.change_body_type(ct),
            Self::AuthEditorAction(action) => action.update(request),
            Self::OpenFilePicker => {
                return open_file_dialog("Select File", |path| {
                    RequestPaneMsg::ChangeBodyFile(path.map(|p| p.path().to_path_buf()))
                });
            }
        };
        Command::none()
    }
}

fn params_view(request: &RequestPane) -> iced::Element<RequestPaneMsg> {
    let query = key_value_editor(&request.query_params).on_change(RequestPaneMsg::Queries);

    let has_params = request.path_params.size() > 0;
    let path = has_params.then(|| {
        Column::new()
            .push("Path Params")
            .push(key_value_editor(&request.path_params).on_change(RequestPaneMsg::PathParams))
            .spacing(4)
    });

    Column::new()
        .push(Column::new().push("Query Params").push(query).spacing(4))
        .push_maybe(path)
        .spacing(8)
        .into()
}

fn headers_view(request: &RequestPane) -> iced::Element<RequestPaneMsg> {
    Column::new()
        .push("Headers")
        .push(key_value_editor(&request.headers).on_change(RequestPaneMsg::Headers))
        .spacing(4)
        .into()
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestPaneMsg> {
    let request = &state.active_tab().request();

    let tab_content = match request.tab {
        ReqTabId::Params => params_view(request),
        ReqTabId::Headers => headers_view(request),
        ReqTabId::Auth => auth_view(request).map(RequestPaneMsg::AuthEditorAction),
        ReqTabId::Body => body_tab(&request.body),
    };

    let tabs = button_tabs(
        request.tab,
        [
            button_tab(ReqTabId::Params, || text("Params")),
            button_tab(ReqTabId::Auth, || text("Auth")),
            button_tab(ReqTabId::Body, || text("Body")),
            button_tab(ReqTabId::Headers, || text("Headers")),
        ]
        .into_iter(),
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
