use chrono::{DateTime, Local};
use components::{colors, icon, icon_button, icons, table, tooltip};
use core::persistence::history::{HistoryEntry, HistoryEntrySummary};
use core::utils::fmt_duration;
use humansize::{BINARY, format_size};
use iced::widget::{Space, button, column, row, text, text_input};
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
    OpenHistoryEntry(Option<HistoryEntry>),
    SearchChanged(String),
    LoadComplete(Vec<HistoryEntrySummary>),
    ClearSearch,
}

impl HistoryTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            HistoryTabMsg::OpenEntry(id) => {
                let history_db = state.common.history_db.clone();
                if let Some(db) = history_db {
                    return Task::future(async move {
                        match db.get_history_by_id(id).await {
                            Ok(Some(entry)) => HistoryTabMsg::OpenHistoryEntry(Some(entry)),
                            Ok(None) => HistoryTabMsg::OpenHistoryEntry(None),
                            Err(e) => {
                                log::error!("Error loading history entry: {e:?}");
                                HistoryTabMsg::OpenHistoryEntry(None)
                            }
                        }
                    });
                }
                Task::none()
            }
            HistoryTabMsg::OpenHistoryEntry(entry) => {
                if let Some(entry) = entry {
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
            HistoryTabMsg::SearchChanged(query) => {
                let Some(Tab::History(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.set_search_query(query.clone());

                if query.trim().is_empty() {
                    let history_db = state.common.history_db.clone();
                    if let Some(db) = history_db {
                        return Task::future(async move {
                            match db.get_history_summary(Some(100)).await {
                                Ok(entries) => HistoryTabMsg::LoadComplete(entries),
                                Err(e) => {
                                    log::error!("Error loading history: {e:?}");
                                    HistoryTabMsg::LoadComplete(vec![])
                                }
                            }
                        });
                    }
                }

                Task::none()
            }
            HistoryTabMsg::LoadComplete(entries) => {
                let Some(Tab::History(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.set_entries(entries);
                Task::none()
            }
            HistoryTabMsg::ClearSearch => {
                let Some(Tab::History(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.clear_search_query();

                // Load all results after clearing search
                let history_db = state.common.history_db.clone();
                if let Some(db) = history_db {
                    return Task::future(async move {
                        match db.get_history_summary(Some(100)).await {
                            Ok(entries) => HistoryTabMsg::LoadComplete(entries),
                            Err(e) => {
                                log::error!("Error loading history: {e:?}");
                                HistoryTabMsg::LoadComplete(vec![])
                            }
                        }
                    });
                }
                Task::none()
            }
        }
    }
}

pub fn view<'a>(_state: &'a AppState, tab: &'a HistoryTab) -> Element<'a, HistoryTabMsg> {
    let search_placeholder = if tab.is_searching {
        "Searching..."
    } else {
        "Search history (method, URL, body, description)..."
    };

    let search_input = text_input(search_placeholder, &tab.search_query)
        .on_input(HistoryTabMsg::SearchChanged)
        .padding(8)
        .size(14)
        .width(Length::FillPortion(1));

    let clear_search_button = icon_button(icons::Close, Some(24), Some(10))
        .style(button::secondary)
        .on_press_maybe(
            tab.search_query
                .is_empty()
                .then_some(HistoryTabMsg::ClearSearch),
        );

    let clear_history_button = icon_button(icons::Delete, Some(24), Some(10))
        .style(button::danger)
        .on_press(HistoryTabMsg::ClearHistory);

    let search_row = row![search_input, clear_search_button, clear_history_button]
        .height(Length::Shrink)
        .width(Length::Fill)
        .align_y(iced::Alignment::Center)
        .spacing(5);

    let content: Element<'a, HistoryTabMsg> = if let Some(error) = &tab.error {
        Element::from(text(format!("Error: {error}")))
    } else if tab.entries.is_empty() {
        let message = if tab.search_query.trim().is_empty() {
            "No history entries found"
        } else {
            "No matching history entries found"
        };
        Element::from(text(message))
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

    column![search_row, content]
        .spacing(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
