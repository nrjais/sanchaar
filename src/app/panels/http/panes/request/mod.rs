use lib::http::collection::Collection;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use iced::padding;
use iced::widget::{Column, Row, button, scrollable, space};
use iced::{Length, Task, widget::text};

use crate::commands::dialog::open_file_dialog;
use crate::components::{CodeEditorMsg, FilePickerUpdateMsg};
use crate::components::{
    FilePickerAction, KeyValUpdateMsg, button_tab, button_tabs, icon_button, icons,
    key_value_editor,
};

use crate::state::request::{BulkEditMsg, ReqTabId, ScriptType as ReqScriptType};
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
    ChangePostRequestScript(Option<String>),
    ChangeScriptType(ReqScriptType),
    LoadScriptContent(String),
    ScriptEditorAction(CodeEditorMsg),
    SaveScript,
    ScriptSaved,
    OpenFilePicker,
    FormatBody,
}

impl RequestPaneMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Http(http_tab)) = state.active_tab_mut() else {
            return Task::none();
        };
        let request = http_tab.request_mut();

        match self {
            Self::TabSelected(tab) => {
                request.tab = tab;

                // When switching to Script tab, load the script content if not already loaded
                if tab == ReqTabId::PreRequest && request.script_content.is_none() {
                    let script_name = match request.script_type {
                        ReqScriptType::PreRequest => request.pre_request.clone(),
                        ReqScriptType::PostRequest => request.post_request.clone(),
                    };

                    if let Some(script_name) = script_name {
                        let col_key = http_tab.collection_key();
                        let col = state.common.collections.get(col_key);
                        if let Some(col) = col
                            && let Some(path) = col.get_script_path(&script_name)
                        {
                            return Task::perform(tokio::fs::read_to_string(path), |result| {
                                match result {
                                    Ok(content) => RequestPaneMsg::LoadScriptContent(content),
                                    Err(e) => {
                                        log::error!("Failed to load script: {}", e);
                                        RequestPaneMsg::LoadScriptContent(String::new())
                                    }
                                }
                            });
                        }
                    }
                }
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
                request.pre_request = script.clone();
                request.script_type = ReqScriptType::PreRequest;
                // Load script content if a script is selected
                if let Some(script_name) = script {
                    let col_key = http_tab.collection_key();
                    let col = state.common.collections.get(col_key);
                    if let Some(col) = col
                        && let Some(path) = col.get_script_path(&script_name)
                    {
                        return Task::perform(
                            tokio::fs::read_to_string(path),
                            |result| match result {
                                Ok(content) => RequestPaneMsg::LoadScriptContent(content),
                                Err(e) => {
                                    log::error!("Failed to load script: {}", e);
                                    RequestPaneMsg::LoadScriptContent(String::new())
                                }
                            },
                        );
                    }
                } else {
                    request.script_content = None;
                    request.script_edited = false;
                }
            }
            Self::ChangePostRequestScript(script) => {
                request.post_request = script.clone();
                request.script_type = ReqScriptType::PostRequest;
                // Load script content if a script is selected
                if let Some(script_name) = script {
                    let col_key = http_tab.collection_key();
                    let col = state.common.collections.get(col_key);
                    if let Some(col) = col
                        && let Some(path) = col.get_script_path(&script_name)
                    {
                        return Task::perform(
                            tokio::fs::read_to_string(path),
                            |result| match result {
                                Ok(content) => RequestPaneMsg::LoadScriptContent(content),
                                Err(e) => {
                                    log::error!("Failed to load script: {}", e);
                                    RequestPaneMsg::LoadScriptContent(String::new())
                                }
                            },
                        );
                    }
                } else {
                    request.script_content = None;
                    request.script_edited = false;
                }
            }
            Self::ChangeScriptType(script_type) => {
                request.script_type = script_type;
                // Load the content for the selected script type
                let script_name = match script_type {
                    ReqScriptType::PreRequest => request.pre_request.clone(),
                    ReqScriptType::PostRequest => request.post_request.clone(),
                };
                if let Some(script_name) = script_name {
                    let col_key = http_tab.collection_key();
                    let col = state.common.collections.get(col_key);
                    if let Some(col) = col
                        && let Some(path) = col.get_script_path(&script_name)
                    {
                        return Task::perform(
                            tokio::fs::read_to_string(path),
                            |result| match result {
                                Ok(content) => RequestPaneMsg::LoadScriptContent(content),
                                Err(e) => {
                                    log::error!("Failed to load script: {}", e);
                                    RequestPaneMsg::LoadScriptContent(String::new())
                                }
                            },
                        );
                    }
                } else {
                    request.script_content = None;
                    request.script_edited = false;
                }
            }
            Self::LoadScriptContent(content) => {
                use crate::components::editor;
                request.script_content = Some(editor::Content::with_text(&content));
                request.script_edited = false;
            }
            Self::ScriptEditorAction(msg) => {
                if let Some(content) = &mut request.script_content {
                    msg.update(content);
                    request.script_edited = true;
                }
            }
            Self::SaveScript => {
                let script_name = match request.script_type {
                    ReqScriptType::PreRequest => request.pre_request.clone(),
                    ReqScriptType::PostRequest => request.post_request.clone(),
                };
                if let Some(script_name) = script_name
                    && let Some(content) = &request.script_content
                {
                    let text = content.text();
                    let col_key = http_tab.collection_key();
                    let col = state.common.collections.get(col_key);
                    if let Some(col) = col
                        && let Some(path) = col.get_script_path(&script_name)
                    {
                        return Task::perform(
                            tokio::fs::write(path.clone(), text),
                            move |result| match result {
                                Ok(_) => {
                                    log::info!("Script saved successfully");
                                    RequestPaneMsg::ScriptSaved
                                }
                                Err(e) => {
                                    log::error!("Failed to save script: {}", e);
                                    RequestPaneMsg::ScriptSaved
                                }
                            },
                        );
                    }
                }
            }
            Self::ScriptSaved => {
                request.script_edited = false;
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
    use crate::components::{
        button_tab, script_editor, script_placeholder, script_selector, vertical_button_tabs,
    };
    use iced::widget::container;

    let Some(col) = col else {
        return Column::new().into();
    };

    let scripts = &col.scripts;
    let request = tab.request();
    let script_type = request.script_type;

    // Get the selected script based on current script type
    let selected = match script_type {
        ReqScriptType::PreRequest => request.pre_request.as_ref(),
        ReqScriptType::PostRequest => request.post_request.as_ref(),
    };

    // Script type vertical tabs
    let script_type_tabs = vertical_button_tabs(
        script_type,
        [
            button_tab(ReqScriptType::PreRequest, || text("Pre-request").size(12)),
            button_tab(ReqScriptType::PostRequest, || {
                text("Post-response").size(12)
            }),
        ]
        .into_iter(),
        RequestPaneMsg::ChangeScriptType,
    );

    // Script selector with remove/save buttons (no create)
    let selector = script_selector(
        scripts,
        selected,
        move |s| match script_type {
            ReqScriptType::PreRequest => RequestPaneMsg::ChangePreRequestScript(Some(s)),
            ReqScriptType::PostRequest => RequestPaneMsg::ChangePostRequestScript(Some(s)),
        },
        Some(match script_type {
            ReqScriptType::PreRequest => RequestPaneMsg::ChangePreRequestScript(None),
            ReqScriptType::PostRequest => RequestPaneMsg::ChangePostRequestScript(None),
        }),
        None, // Remove create action
        selected.and_then(|_| {
            request
                .script_content
                .as_ref()
                .map(|_| (request.script_edited, RequestPaneMsg::SaveScript))
        }),
    );

    // Editor or placeholder
    let editor_view: iced::Element<'a, RequestPaneMsg> = if selected.is_some() {
        if let Some(script_content) = request.script_content.as_ref() {
            script_editor(script_content, true, RequestPaneMsg::ScriptEditorAction)
        } else {
            script_placeholder("Loading script...")
        }
    } else {
        script_placeholder("Select a script to edit")
    };

    let content_view = Column::new()
        .push(selector)
        .push(editor_view)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(0);

    Row::new()
        .push(
            container(script_type_tabs)
                .padding(8)
                .width(Length::Fixed(140.0))
                .height(Length::Fill),
        )
        .push(content_view)
        .width(Length::Fill)
        .height(Length::Fill)
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
