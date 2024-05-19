use core::http::{collection, CollectionKey};
use std::path::PathBuf;

use iced::widget::{button, horizontal_space, pick_list, scrollable, Column, Row};
use iced::{widget::text, Command, Length};

use crate::commands::dialog::open_file_dialog;
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::request::{RawRequestBody, RequestPane};
use crate::state::{request::ReqTabId, AppState};
use components::{
    button_tab, button_tabs, icon_button, icons, key_value_editor, tooltip, KeyValUpdateMsg,
};
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
    MulitpartOpenFilePicker(usize),
    ChangeBodyFile(Option<PathBuf>),
    ChangeBodyType(&'static str),
    ChangePreRequestScript(Option<collection::Script>),
    OpenFilePicker,
    CreateScript(CollectionKey),
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
            Self::ChangePreRequestScript(script) => {
                request.pre_request = script;
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
            Self::MulitpartOpenFilePicker(idx) => {
                return open_file_dialog("Select File", move |handle| {
                    let path = handle.map(|p| p.path().to_path_buf());
                    RequestPaneMsg::MultipartFilesAction(FilePickerUpdateMsg::FilePicked(idx, path))
                });
            }
            Self::ChangeBodyType(ct) => request.change_body_type(ct),
            Self::AuthEditorAction(action) => action.update(request),
            Self::OpenFilePicker => {
                return open_file_dialog("Select File", |path| {
                    RequestPaneMsg::ChangeBodyFile(path.map(|p| p.path().to_path_buf()))
                });
            }
            Self::CreateScript(col) => {
                Popup::popup_name(state, String::new(), PopupNameAction::NewScript(col));
            }
        };
        Command::none()
    }
}

fn params_view(request: &RequestPane) -> iced::Element<RequestPaneMsg> {
    let has_params = request.path_params.size() > 0;
    let path = has_params.then(|| {
        Column::new()
            .push("Path Params")
            .push(key_value_editor(&request.path_params).on_change(RequestPaneMsg::PathParams))
            .width(Length::Fill)
            .spacing(4)
    });

    let query = Column::new()
        .push("Query Params")
        .push(key_value_editor(&request.query_params).on_change(RequestPaneMsg::Queries))
        .spacing(4)
        .width(Length::Fill);

    scrollable(Column::new().push(query).push_maybe(path).spacing(8))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn headers_view(request: &RequestPane) -> iced::Element<RequestPaneMsg> {
    scrollable(
        Column::new()
            .push("Headers")
            .push(key_value_editor(&request.headers).on_change(RequestPaneMsg::Headers))
            .width(Length::Fill)
            .spacing(4),
    )
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

fn script_view(state: &AppState) -> iced::Element<RequestPaneMsg> {
    let tab = state.active_tab();
    let Some(collection_key) = tab.collection_key() else {
        return Column::new().into();
    };

    let scripts = state
        .collections
        .get(collection_key)
        .map(|c| &c.scripts)
        .cloned()
        .unwrap_or_default();

    let selected = tab.request().pre_request.as_ref();

    Column::new()
        .push(
            Row::new()
                .push(text("Pre-Request Script"))
                .push(horizontal_space())
                .push(tooltip(
                    "New Script",
                    icon_button(icons::Plus, None, Some(4))
                        .on_press(RequestPaneMsg::CreateScript(collection_key))
                        .style(button::secondary),
                )),
        )
        .push(
            Row::new()
                .push(
                    pick_list(scripts, selected, |s| {
                        RequestPaneMsg::ChangePreRequestScript(Some(s))
                    })
                    .placeholder("Select Script")
                    .width(Length::Fill)
                    .padding([2, 6])
                    .text_size(16),
                )
                .push(
                    icon_button(icons::Close, Some(20), Some(6))
                        .on_press_maybe(
                            selected.map(|_| RequestPaneMsg::ChangePreRequestScript(None)),
                        )
                        .style(button::secondary),
                )
                .spacing(4),
        )
        .width(Length::Fill)
        .spacing(8)
        .into()
}

pub(crate) fn view(state: &AppState) -> iced::Element<RequestPaneMsg> {
    let request = state.active_tab().request();
    let col = state.active_tab().collection_key();

    let tab_content = match request.tab {
        ReqTabId::Params => params_view(request),
        ReqTabId::Headers => headers_view(request),
        ReqTabId::Auth => auth_view(request).map(RequestPaneMsg::AuthEditorAction),
        ReqTabId::Body => body_tab(&request.body),
        ReqTabId::PreRequest => script_view(state),
    };

    let tabs = button_tabs(
        request.tab,
        [
            button_tab(ReqTabId::Params, || text("Params")),
            button_tab(ReqTabId::Auth, || text("Auth")),
            button_tab(ReqTabId::Body, || text("Body")),
            button_tab(ReqTabId::Headers, || text("Headers")),
        ]
        .into_iter()
        .chain(
            col.map(|_| button_tab(ReqTabId::PreRequest, || text("Script")))
                .into_iter(),
        ),
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
