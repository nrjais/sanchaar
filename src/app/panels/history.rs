use chrono::{DateTime, Local};
use components::{colors, icon, icons, table, tooltip};
use core::utils::fmt_duration;
use humansize::{BINARY, format_size};
use iced::widget::{Space, button, column, row, text};
use iced::{Element, Length, Task};
use std::time::Duration;

use crate::state::tabs::history_tab::HistoryTab;
use crate::state::{AppState, HttpTab, Tab};
use core::http::CollectionRequest;

#[derive(Debug, Clone)]
pub enum HistoryTabMsg {
    OpenEntry(i64),
    DeleteEntry(i64),
    ClearHistory,
}

impl HistoryTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            HistoryTabMsg::OpenEntry(id) => {
                let Some(Tab::History(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                if let Some(entry) = tab.entries.iter().find(|e| e.id == id) {
                    if let (Ok(request), Ok(response)) = (entry.to_request(), entry.to_response()) {
                        let tab_name = format!("{} {}", entry.method, entry.url);
                        let collection_ref = CollectionRequest::default();
                        let new_tab =
                            HttpTab::from_history(&tab_name, request, response, collection_ref);
                        state.open_tab(Tab::Http(new_tab));
                    }
                }
                Task::none()
            }
            HistoryTabMsg::DeleteEntry(id) => {
                let history_db = state.common.history_db.clone();
                if let Some(db) = history_db {
                    return Task::future(async move { db.delete_history_entry(id).await })
                        .discard();
                }
                Task::none()
            }
            HistoryTabMsg::ClearHistory => {
                let history_db = state.common.history_db.clone();
                if let Some(db) = history_db {
                    return Task::future(async move { db.clear_history().await }).discard();
                }
                Task::none()
            }
        }
    }
}

pub fn view<'a>(_state: &'a AppState, tab: &'a HistoryTab) -> Element<'a, HistoryTabMsg> {
    let actions = row![
        button(text("Clear All"))
            .style(button::danger)
            .on_press(HistoryTabMsg::ClearHistory),
    ]
    .padding(10);

    let content: Element<'a, HistoryTabMsg> = if let Some(error) = &tab.error {
        Element::from(text(format!("Error: {error}")))
    } else if tab.entries.is_empty() {
        Element::from(text("No history entries found"))
    } else {
        let headers = [
            text("Method").size(14).into(),
            text("URL").size(14).into(),
            text("Status").size(14).into(),
            text("Duration").size(14).into(),
            text("Size").size(14).into(),
            text("Time").size(14).into(),
            text("Actions").size(14).into(),
        ];

        let rows = tab
            .entries
            .iter()
            .map(|entry| {
                let local_time: DateTime<Local> = entry.timestamp.into();
                let duration = Duration::from_millis(entry.response_duration_ms as u64);

                let method_color = match entry.method.as_str() {
                    "GET" => colors::GREEN,
                    "POST" => colors::BLUE,
                    "PUT" => colors::ORANGE,
                    "DELETE" => colors::RED,
                    "PATCH" => colors::PURPLE,
                    _ => colors::DARK_GREY,
                };

                let status_color = match entry.response_status {
                    200..=299 => colors::GREEN,
                    300..=399 => colors::ORANGE,
                    400..=499 => colors::RED,
                    500..=599 => colors::DARK_RED,
                    _ => colors::DARK_GREY,
                };

                let actions = row![
                    tooltip(
                        "Open in new tab",
                        button(icon(icons::Send))
                            .style(button::secondary)
                            .on_press(HistoryTabMsg::OpenEntry(entry.id))
                    ),
                    Space::with_width(Length::Fixed(5.0)),
                    tooltip(
                        "Delete entry",
                        button(icon(icons::Delete))
                            .style(button::danger)
                            .on_press(HistoryTabMsg::DeleteEntry(entry.id))
                    ),
                ];

                [
                    text(&entry.method).size(14).color(method_color).into(),
                    text(&entry.url).size(14).into(),
                    text(entry.response_status.to_string())
                        .size(14)
                        .color(status_color)
                        .into(),
                    text(fmt_duration(duration)).size(14).into(),
                    text(format_size(entry.response_size_bytes as u64, BINARY))
                        .size(14)
                        .into(),
                    text(local_time.format("%m/%d %H:%M:%S").to_string())
                        .size(14)
                        .into(),
                    actions.into(),
                ]
            })
            .collect::<Vec<_>>();

        let widths = [8, 30, 8, 10, 10, 15, 15];

        Element::from(table(headers, rows, widths))
    };

    column![actions, content,]
        .spacing(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
