use lib::http::CollectionKey;
use lib::http::collection::Collection;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use iced::padding;
use iced::widget::{Column, Row, button, pick_list, scrollable, space};
use iced::{Length, Task, widget::text};

use crate::commands::dialog::open_file_dialog;
use crate::components::{CodeEditorMsg, FilePickerUpdateMsg};
use crate::components::{
    FilePickerAction, KeyValUpdateMsg, button_tab, button_tabs, icon_button, icons,
    key_value_editor, tooltip,
};
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::request::{BulkEditMsg, ReqTabId};
use crate::state::request::{RawRequestBody, RequestPane};
use crate::state::{AppState, HttpTab, Tab};

use self::auth_editor::{AuthEditorMsg, auth_view};
use self::body_view::body_tab;

mod auth_editor;
mod body_editor;
mod body_view;
mod bulk_edit;

#[derive(Debug, Clone)]
pub enum RequestPaneMsg {
    TabSelected(ReqTabId),
    Headers(BulkEditMsg),
    Queries(BulkEditMsg),
    PathParams(KeyValUpdateMsg),
    BodyEditorAction(CodeEditorMsg),
    AuthEditorAction(AuthEditorMsg),
    FormBodyEditAction(KeyValUpdateMsg),
    MultipartParamsAction(KeyValUpdateMsg),
    MultipartFilesAction(FilePickerUpdateMsg),
    ChangeBodyFile(Option<PathBuf>),
    ChangeBodyType(&'static str),
    ChangePreRequestScript(Option<String>),
    OpenFilePicker,
    CreateScript(CollectionKey),
    FormatBody,
}

impl RequestPaneMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Http(tab)) = state.active_tab_mut() else {
            return Task::none();
        };
        let request = tab.request_mut();

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
            Self::MultipartFilesAction(FilePickerUpdateMsg::OpenFilePicker(idx)) => {
                return open_file_dialog("Select File").map(move |handle| {
                    let path = handle.map(|p| p.path().to_path_buf());
                    RequestPaneMsg::MultipartFilesAction(FilePickerUpdateMsg::Action(
                        FilePickerAction::FilePicked(idx, path),
                    ))
                });
            }
            Self::MultipartFilesAction(FilePickerUpdateMsg::Action(action)) => {
                if let RawRequestBody::Multipart(_, files) = &mut request.body {
                    files.update(action);
                }
            }
            Self::ChangeBodyFile(path) => {
                request.body = RawRequestBody::File(path);
            }
            Self::ChangeBodyType(ct) => request.change_body_type(ct),
            Self::FormatBody => request.format_body(),
            Self::AuthEditorAction(action) => action.update(request),
            Self::OpenFilePicker => {
                return open_file_dialog("Select File").map(|path| {
                    RequestPaneMsg::ChangeBodyFile(path.map(|p| p.path().to_path_buf()))
                });
            }
            Self::CreateScript(col) => {
                Popup::popup_name(
                    &mut state.common,
                    String::new(),
                    PopupNameAction::NewScript(col),
                );
            }
        };
        Task::none()
    }
}

fn bulk_edit_toggle<'a>(
    title: &'a str,
    msg: RequestPaneMsg,
    is_editor: bool,
) -> Row<'a, RequestPaneMsg> {
    let icon = if is_editor {
        icons::EditLines
    } else {
        icons::Edit
    };

    Row::new()
        .push(title)
        .push(space::horizontal().width(Length::Fixed(8.)))
        .push(
            icon_button(icon, None, Some(4))
                .style(button::text)
                .on_press(msg),
        )
}

fn params_view(request: &RequestPane, vars: Arc<HashSet<String>>) -> iced::Element<RequestPaneMsg> {
    let has_params = request.path_params.size() > 0;
    let path = has_params.then(|| {
        let editor =
            key_value_editor(&request.path_params, &vars).on_change(RequestPaneMsg::PathParams);
        Column::new().push("Path Params").push(editor).spacing(4)
    });

    let query = Column::new()
        .push(bulk_edit_toggle(
            "Query Params",
            RequestPaneMsg::Queries(BulkEditMsg::ToggleMode),
            request.query_params.is_editor(),
        ))
        .push(bulk_edit::view(&request.query_params, vars, false).map(RequestPaneMsg::Queries))
        .spacing(4);

    scrollable(
        Column::new()
            .push(path)
            .push(query)
            .spacing(8)
            .padding(padding::right(12)),
    )
    .into()
}

fn headers_view(
    request: &RequestPane,
    vars: Arc<HashSet<String>>,
) -> iced::Element<RequestPaneMsg> {
    Column::new()
        .push(bulk_edit_toggle(
            "Headers",
            RequestPaneMsg::Headers(BulkEditMsg::ToggleMode),
            request.headers.is_editor(),
        ))
        .push(bulk_edit::view(&request.headers, vars, true).map(RequestPaneMsg::Headers))
        .width(Length::Fill)
        .spacing(4)
        .into()
}

fn script_view<'a>(
    col: Option<&'a Collection>,
    tab: &'a HttpTab,
) -> iced::Element<'a, RequestPaneMsg> {
    let Some(col) = col else {
        return Column::new().into();
    };

    let scripts = &col.scripts;
    let selected = tab.request().pre_request.as_ref();

    Column::new()
        .push(text("Pre-Request Script"))
        .push(
            pick_list(
                scripts.iter().map(|s| s.name.clone()).collect::<Vec<_>>(),
                selected,
                |s| RequestPaneMsg::ChangePreRequestScript(Some(s)),
            )
            .placeholder("Select Script")
            .width(Length::Fill)
            .padding([2, 8])
            .text_size(16),
        )
        .push(
            Row::new()
                .push(space::horizontal())
                .push(tooltip(
                    "New Script",
                    icon_button(icons::Plus, Some(20), Some(12))
                        .on_press(RequestPaneMsg::CreateScript(tab.collection_key()))
                        .style(button::secondary),
                ))
                .push(tooltip(
                    "Remove Script",
                    icon_button(icons::Close, Some(20), Some(12))
                        .on_press_maybe(
                            selected.map(|_| RequestPaneMsg::ChangePreRequestScript(None)),
                        )
                        .style(button::secondary),
                ))
                .push(space::horizontal())
                .width(Length::Fill)
                .align_y(iced::Alignment::Center)
                .spacing(4),
        )
        .width(Length::Fill)
        .spacing(8)
        .into()
}

pub fn view<'a>(
    tab: &'a HttpTab,
    col: Option<&'a Collection>,
) -> iced::Element<'a, RequestPaneMsg> {
    let request = tab.request();

    let vars = col.map(|c| c.env_chain().all_var_set()).unwrap_or_default();

    let tab_content = match request.tab {
        ReqTabId::Params => params_view(request, Arc::clone(&vars)),
        ReqTabId::Headers => headers_view(request, Arc::clone(&vars)),
        ReqTabId::Auth => {
            auth_view(request, Arc::clone(&vars)).map(RequestPaneMsg::AuthEditorAction)
        }
        ReqTabId::Body => body_tab(&request.body, vars),
        ReqTabId::PreRequest => script_view(col, tab),
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
        .chain(col.map(|_| button_tab(ReqTabId::PreRequest, || text("Script")))),
        RequestPaneMsg::TabSelected,
        None,
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(8)
        .padding([4, 0])
        .into()
}
