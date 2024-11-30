use core::utils::fmt_duration;
use std::sync::Arc;

use humansize::{format_size, BINARY};
use iced::widget::{button, container, text, text_input, Column, Row};
use iced::{clipboard, Alignment, Border, Color, Element, Task, Theme};

use components::{
    button_tab, button_tabs, code_editor, key_value_viewer, CodeEditorMsg, ContentType,
};

use crate::commands::builders::write_file_cmd;
use crate::commands::dialog::create_file_dialog;
use crate::state::response::ResponseTabId;
use crate::state::response::{BodyMode, CompletedResponse, ResponseState};
use crate::state::HttpTab;

#[derive(Debug, Clone)]
pub enum CompletedMsg {
    TabChanged(ResponseTabId),
    CodeViewerMsg(CodeEditorMsg),
    SetBodyMode(BodyMode),
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
            CompletedMsg::SetBodyMode(mode) => {
                res.mode = mode;
            }
            CompletedMsg::CopyBodyToClipboard => {
                return clipboard::write(res.selected_content().text());
            }
            CompletedMsg::SaveResponse => {
                return create_file_dialog("Save response body")
                    .map(|path| CompletedMsg::SaveToFile(path));
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
    fn button_style(theme: &Theme, _status: button::Status, selected: bool) -> button::Style {
        if selected {
            button::secondary(theme, button::Status::Active)
        } else {
            button::text(theme, button::Status::Active)
        }
    }

    let mode = cr.mode;
    let size = 14;
    let actions = Row::new()
        .push(
            button(text("Preview").size(size))
                .padding([2, 4])
                .on_press(CompletedMsg::SetBodyMode(BodyMode::Pretty))
                .style(move |t, s| button_style(t, s, BodyMode::Pretty == mode)),
        )
        .push(
            button(text("Raw").size(size))
                .padding([2, 4])
                .on_press(CompletedMsg::SetBodyMode(BodyMode::Raw))
                .style(move |t, s| button_style(t, s, BodyMode::Raw == mode)),
        )
        .spacing(2);

    let json_path_filter = text_input("$.filter", &cr.json_path_filter)
        .on_input(CompletedMsg::JsonPathFilter)
        .on_paste(CompletedMsg::JsonPathFilter);

    let action_bar = Row::new()
        .push(container(actions).style(|theme: &Theme| {
            container::Style {
                border: Border::default()
                    .width(1)
                    .color(theme.extended_palette().background.weak.color),
                ..container::transparent(theme)
            }
        }))
        .push(
            button(text("Copy").size(size))
                .padding([2, 4])
                .style(button::secondary)
                .on_press(CompletedMsg::CopyBodyToClipboard),
        )
        .push(
            button(text("Save").size(size))
                .padding([2, 4])
                .style(button::secondary)
                .on_press(CompletedMsg::SaveResponse),
        )
        .spacing(8);

    let content = cr.selected_content();
    let is_json = cr.result.body.is_json();
    let content_type = if is_json {
        ContentType::Json
    } else {
        ContentType::Text
    };

    Column::new()
        .push(action_bar)
        .push(code_editor(content, content_type).map(CompletedMsg::CodeViewerMsg))
        .push_maybe(is_json.then_some(json_path_filter))
        .spacing(4)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}

pub fn view<'a>(tab: &'a HttpTab, cr: &'a CompletedResponse) -> Element<'a, CompletedMsg> {
    let res = &cr.result;

    let status_size = 12;
    let status = Row::new()
        .push(
            text(res.status.to_string())
                .size(status_size)
                .color(status_color(res.status)),
        )
        .push(
            text(format_size(res.size_bytes, BINARY))
                .size(status_size)
                .color(Color::from_rgb8(182, 128, 182)),
        )
        .push(
            text(fmt_duration(res.duration))
                .size(status_size)
                .color(Color::from_rgb8(160, 160, 160)),
        )
        .padding([4, 8])
        .spacing(8)
        .align_y(Alignment::Center);

    let headers = res
        .headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or_default()))
        .collect::<Vec<_>>();

    let tab_content = match tab.response.active_tab {
        ResponseTabId::Body => body_view(cr),
        ResponseTabId::Headers => key_value_viewer(&headers),
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
