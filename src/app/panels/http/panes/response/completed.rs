use lib::client;
use lib::utils::fmt_duration;
use std::sync::Arc;

use humansize::{BINARY, format_size};
use iced::widget::{Column, Row, button, container, space, text};
use iced::{Alignment, Color, Element, Length, Task, Theme, clipboard};

use crate::components::editor::Content;
use crate::components::{
    CodeEditorMsg, ContentType, LineEditorMsg, button_tab, button_tabs, code_editor, colors, icon,
    icons, key_value_viewer, line_editor, tooltip,
};

use crate::commands::builders::write_file_cmd;
use crate::commands::dialog::create_file_dialog;
use crate::state::HttpTab;
use crate::state::response::ResponseTabId;
use crate::state::response::{BodyMode, CompletedResponse, ResponseState};
use crate::state::utils::headers_to_string;

#[derive(Debug, Clone)]
pub enum CompletedMsg {
    TabChanged(ResponseTabId),
    CodeViewerMsg(BodyMode, CodeEditorMsg),
    CopyBodyToClipboard(BodyMode),
    SaveResponse,
    SaveToFile(Option<Arc<rfd::FileHandle>>),
    Done,
    JsonPathFilter(LineEditorMsg),
    CopyHeadersToClipboard,
    ToggleJsonPathFilter,
}

impl CompletedMsg {
    pub fn update(self, tab: &mut HttpTab) -> Task<CompletedMsg> {
        let ResponseState::Completed(ref mut res) = tab.response.state else {
            return Task::none();
        };

        match self {
            CompletedMsg::TabChanged(key) => {
                tab.response.active_tab = key;
            }
            CompletedMsg::CodeViewerMsg(mode, msg) => {
                msg.update(res.selected_content_mut(mode));
            }
            CompletedMsg::CopyBodyToClipboard(mode) => {
                return clipboard::write(res.selected_content(mode).text());
            }
            CompletedMsg::SaveResponse => {
                return create_file_dialog("Save response body").map(CompletedMsg::SaveToFile);
            }
            CompletedMsg::SaveToFile(path) => {
                if let Some(path) = path {
                    let body = &res.result.body.data;
                    return write_file_cmd(Arc::clone(body), path).map(|_| CompletedMsg::Done);
                }
            }
            CompletedMsg::Done => (),
            CompletedMsg::JsonPathFilter(action) => {
                if let Some(json_path_filter) = res.json_path_filter.as_mut() {
                    action.update(json_path_filter);
                }
                res.apply_json_path_filter();
            }
            CompletedMsg::CopyHeadersToClipboard => {
                return clipboard::write(headers_to_string(&res.result.headers));
            }
            CompletedMsg::ToggleJsonPathFilter => {
                if res.json_path_filter.is_none() {
                    res.json_path_filter = Some(Content::new());
                } else {
                    res.json_path_filter = None;
                }
                res.apply_json_path_filter();
            }
        }
        Task::none()
    }
}

fn status_color(status: reqwest::StatusCode) -> Color {
    match status.as_u16() {
        200..=299 => Color::from_rgb8(0, 200, 0),
        300..=399 => Color::from_rgb8(0, 0, 200),
        400..=499 => Color::from_rgb8(200, 200, 0),
        500..=599 => Color::from_rgb8(200, 0, 0),
        _ => Color::WHITE,
    }
}

fn body_view(cr: &CompletedResponse, mode: BodyMode) -> Element<CompletedMsg> {
    let json_path_filter = cr
        .json_path_filter
        .as_ref()
        .map(|filter| -> Element<CompletedMsg> {
            line_editor(filter)
                .placeholder("$.filter")
                .highlight(false)
                .map(CompletedMsg::JsonPathFilter)
        });

    let content = cr.selected_content(mode);
    let is_json = cr.result.body.is_json();
    let content_type = match cr.result.body.content_type {
        client::ContentType::Json => ContentType::Json,
        client::ContentType::Text => ContentType::Text,
        client::ContentType::Html => ContentType::HTML,
        client::ContentType::XML => ContentType::XML,
        client::ContentType::Buffer => ContentType::Text, //TODO: Show buffer correctly
    };

    let editor =
        code_editor(content, content_type).map(move |msg| CompletedMsg::CodeViewerMsg(mode, msg));

    Column::new()
        .push(is_json.then_some(json_path_filter).flatten())
        .push(editor)
        .spacing(4)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}

fn body_actions<'a>(status_size: u32, mode: BodyMode, is_json: bool) -> Row<'a, CompletedMsg> {
    let filter = tooltip(
        "Filter",
        button(icon(icons::Filter).size(status_size))
            .padding([2, 4])
            .style(button::text)
            .on_press(CompletedMsg::ToggleJsonPathFilter),
    );
    let show_filter = is_json && mode == BodyMode::Pretty;
    Row::new()
        .push(show_filter.then_some(filter))
        .push(tooltip(
            "Copy",
            button(icon(icons::Copy).size(status_size))
                .padding([2, 4])
                .style(button::text)
                .on_press(CompletedMsg::CopyBodyToClipboard(mode)),
        ))
        .push(tooltip(
            "Download",
            button(icon(icons::Download).size(status_size))
                .padding([2, 4])
                .style(button::text)
                .on_press(CompletedMsg::SaveResponse),
        ))
}

fn headers_actions<'a>(status_size: u32) -> Row<'a, CompletedMsg> {
    Row::new().push(
        button(icon(icons::Copy).size(status_size))
            .padding([2, 4])
            .style(button::text)
            .on_press(CompletedMsg::CopyHeadersToClipboard),
    )
}

pub fn view<'a>(tab: &'a HttpTab, cr: &'a CompletedResponse) -> Element<'a, CompletedMsg> {
    let res = &cr.result;
    let status_size = 12;

    let status = container(
        text(res.status.to_string())
            .size(status_size)
            .color(status_color(res.status)),
    )
    .style(container::bordered_box)
    .padding([2, 4]);

    let dot = || {
        icon(icons::Dot)
            .color(colors::GREY)
            .size(status_size * 2)
            .line_height(0.5)
            .align_x(Alignment::Center)
    };

    let actions = match tab.response.active_tab {
        ResponseTabId::BodyPreview => {
            body_actions(status_size, BodyMode::Pretty, cr.result.body.is_json())
        }
        ResponseTabId::BodyRaw => {
            body_actions(status_size, BodyMode::Raw, cr.result.body.is_json())
        }
        ResponseTabId::Headers => headers_actions(status_size),
    };
    let actions = actions.spacing(8).padding(0).align_y(Alignment::Center);

    let status = Row::new()
        .push(actions)
        .push(space::horizontal().width(Length::Fixed(8.)))
        .push(status)
        .push(dot())
        .push(
            text(fmt_duration(res.duration))
                .size(status_size)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().success),
                }),
        )
        .push(dot())
        .push(
            text(format_size(res.size_bytes, BINARY))
                .size(status_size)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().success),
                }),
        )
        .spacing(4)
        .padding([0, 4])
        .align_y(Alignment::Center)
        .wrap();

    let tab_content = match tab.response.active_tab {
        ResponseTabId::BodyPreview => body_view(cr, BodyMode::Pretty),
        ResponseTabId::BodyRaw => body_view(cr, BodyMode::Raw),
        ResponseTabId::Headers => {
            let headers = res
                .headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or_default()))
                .collect::<Vec<_>>();
            key_value_viewer(headers)
        }
    };

    let tabs = button_tabs(
        tab.response.active_tab,
        [
            button_tab(ResponseTabId::BodyPreview, || text("Preview")),
            button_tab(ResponseTabId::BodyRaw, || text("Body")),
            button_tab(ResponseTabId::Headers, || text("Headers")),
        ]
        .into_iter(),
        CompletedMsg::TabChanged,
        Some(status.into()),
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .spacing(4)
        .into()
}
