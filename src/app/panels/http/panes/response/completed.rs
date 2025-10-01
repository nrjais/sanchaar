use core::utils::fmt_duration;
use std::sync::Arc;

use humansize::{BINARY, format_size};
use iced::widget::{Column, Row, button, container, space, text, text_input};
use iced::{Alignment, Color, Element, Length, Task, Theme, clipboard};

use crate::components::{
    CodeEditorMsg, ContentType, button_tab, button_tabs, code_editor, colors, icon, icons,
    key_value_viewer,
};

use crate::commands::builders::write_file_cmd;
use crate::commands::dialog::create_file_dialog;
use crate::state::HttpTab;
use crate::state::response::ResponseTabId;
use crate::state::response::{BodyMode, CompletedResponse, ResponseState};

#[derive(Debug, Clone)]
pub enum CompletedMsg {
    TabChanged(ResponseTabId),
    CodeViewerMsg(CodeEditorMsg),
    ToggleBodyMode,
    CopyBodyToClipboard,
    SaveResponse,
    SaveToFile(Option<Arc<rfd::FileHandle>>),
    Done,
    JsonPathFilter(String),
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
            CompletedMsg::CodeViewerMsg(msg) => {
                msg.update(res.selected_content_mut());
            }
            CompletedMsg::ToggleBodyMode => {
                res.mode = match res.mode {
                    BodyMode::Pretty => BodyMode::Raw,
                    BodyMode::Raw => BodyMode::Pretty,
                };
            }
            CompletedMsg::CopyBodyToClipboard => {
                return clipboard::write(res.selected_content().text());
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
            CompletedMsg::JsonPathFilter(filter) => {
                res.json_path_filter = filter;
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

fn body_view(cr: &CompletedResponse) -> Element<CompletedMsg> {
    let json_path_filter = text_input("$.filter", &cr.json_path_filter)
        .on_input(CompletedMsg::JsonPathFilter)
        .on_paste(CompletedMsg::JsonPathFilter);

    let content = cr.selected_content();
    let is_json = cr.result.body.is_json();
    let content_type = if is_json {
        ContentType::Json
    } else {
        ContentType::Text
    };

    Column::new()
        .push(code_editor(content, content_type).map(CompletedMsg::CodeViewerMsg))
        .push(is_json.then_some(json_path_filter))
        .spacing(4)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
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

    let body_mode = if cr.mode == BodyMode::Pretty {
        icons::Preview
    } else {
        icons::NoPreview
    };

    let status = Row::new()
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
        .push(space::horizontal().width(Length::Fixed(4.)))
        .push(
            button(icon(body_mode).size(status_size))
                .padding([2, 4])
                .style(button::text)
                .on_press(CompletedMsg::ToggleBodyMode),
        )
        .push(
            button(icon(icons::Copy).size(status_size))
                .padding([2, 4])
                .style(button::text)
                .on_press(CompletedMsg::CopyBodyToClipboard),
        )
        .push(
            button(icon(icons::Download).size(status_size))
                .padding([2, 4])
                .style(button::text)
                .on_press(CompletedMsg::SaveResponse),
        )
        .spacing(4)
        .padding([0, 4])
        .align_y(Alignment::Center);

    let tab_content = match tab.response.active_tab {
        ResponseTabId::Body => body_view(cr),
        ResponseTabId::Headers => {
            let headers = res
                .headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or_default()))
                .collect::<Vec<_>>();
            key_value_viewer(&headers)
        }
    };

    let tabs = button_tabs(
        tab.response.active_tab,
        [
            button_tab(ResponseTabId::Body, || text("Body")),
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
