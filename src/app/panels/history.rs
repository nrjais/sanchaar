use crate::components::{
    LineEditorMsg, bold, colors, icon, icon_button, icons, line_editor, tooltip,
};
use chrono::{DateTime, Local};
use core::persistence::history::{HistoryEntry, HistoryEntrySummary};
use core::utils::fmt_duration;
use humansize::{BINARY, format_size};
use iced::widget::text::Wrapping;
use iced::widget::{button, column, container, row, scrollable, table, text};
use iced::{Alignment, Element, Length, Task};
use std::time::Duration;

use crate::state::tabs::history_tab::HistoryTab;
use crate::state::{AppState, HttpTab, Tab};
use core::http::CollectionRequest;

#[derive(Debug, Clone)]
pub enum HistoryTabMsg {
    OpenEntry(i64),
    DeleteEntry(i64),
    ClearHistory,
    OpenHistoryEntry(Option<Box<HistoryEntry>>),
    SearchChanged(LineEditorMsg),
    LoadComplete(Vec<HistoryEntrySummary>),
    ClearSearch,
}

fn clear_search_cmd(state: &mut AppState, is_empty: bool) -> Task<HistoryTabMsg> {
    if !is_empty {
        return Task::none();
    }
    let Some(history_db) = state.common.history_db.clone() else {
        return Task::none();
    };
    Task::future(async move {
        match history_db.get_history_summary(Some(100)).await {
            Ok(entries) => HistoryTabMsg::LoadComplete(entries),
            Err(e) => {
                log::error!("Error loading history: {e:?}");
                HistoryTabMsg::LoadComplete(vec![])
            }
        }
    })
}

impl HistoryTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            HistoryTabMsg::OpenEntry(id) => {
                let history_db = state.common.history_db.clone();
                if let Some(db) = history_db {
                    return Task::future(async move {
                        match db.get_history_by_id(id).await {
                            Ok(Some(entry)) => {
                                HistoryTabMsg::OpenHistoryEntry(Some(Box::new(entry)))
                            }
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
                if let Some(entry) = entry
                    && let (Ok(request), Ok(response)) = (entry.to_request(), entry.to_response())
                {
                    let tab_name = format!("{} {}", entry.method, entry.url);
                    let collection_ref = CollectionRequest::default();
                    let new_tab =
                        HttpTab::from_history(&tab_name, request, response, collection_ref);
                    state.open_tab(Tab::Http(new_tab));
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
            HistoryTabMsg::SearchChanged(updat) => {
                let Some(Tab::History(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.set_search_query(updat);
                let is_empty = tab.search_query_text.is_empty();
                clear_search_cmd(state, is_empty)
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
                clear_search_cmd(state, true)
            }
        }
    }
}

pub fn table_view<'a>(tab: &'a HistoryTab) -> Element<'a, HistoryTabMsg> {
    let columns = [
        table::column(bold("When"), |entry: &HistoryEntrySummary| {
            let local_time: DateTime<Local> = entry.timestamp.into();
            text(local_time.format("%m/%d %H:%M:%S").to_string())
        })
        .width(Length::FillPortion(2))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
        table::column(bold("Method"), |entry: &HistoryEntrySummary| {
            let method_color = match entry.method.as_str() {
                "GET" => colors::GREEN,
                "POST" => colors::BLUE,
                "PUT" => colors::ORANGE,
                "DELETE" => colors::RED,
                "PATCH" => colors::PURPLE,
                _ => colors::DARK_GREY,
            };
            text(entry.method.to_string()).color(method_color)
        })
        .width(Length::FillPortion(1))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
        table::column(bold("URL"), |entry: &HistoryEntrySummary| {
            button(text(entry.url.to_string()).wrapping(Wrapping::Glyph))
                .padding([0, 4])
                .style(button::text)
                .on_press(HistoryTabMsg::OpenEntry(entry.id))
                .width(Length::Fill)
        })
        .width(Length::FillPortion(12))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
        table::column(bold("Result"), |entry: &HistoryEntrySummary| {
            let status_color = match entry.response_status {
                200..=299 => colors::GREEN,
                300..=399 => colors::ORANGE,
                400..=499 => colors::RED,
                500..=599 => colors::DARK_RED,
                _ => colors::DARK_GREY,
            };
            let status = text(entry.response_status.to_string())
                .size(12)
                .color(status_color);

            let size = format_size(entry.response_size_bytes as u64, BINARY);
            let size = text(size).size(12).color(colors::DARK_GREY);

            let duration = Duration::from_millis(entry.response_duration_ms as u64);
            let duration = text(fmt_duration(duration))
                .size(12)
                .color(colors::DARK_GREY);

            let dot = || icon(icons::Dot).color(colors::DARK_GREY).size(20);

            row![status, dot(), duration, dot(), size]
                .spacing(4)
                .align_y(Alignment::Center)
                .wrap()
        })
        .width(Length::FillPortion(3))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
        table::column(bold(""), |entry: &HistoryEntrySummary| {
            row![tooltip(
                "Delete entry",
                button(icon(icons::Delete).size(20))
                    .padding([0, 4])
                    .style(button::text)
                    .on_press(HistoryTabMsg::DeleteEntry(entry.id))
            ),]
            .spacing(8)
            .wrap()
        })
        .width(Length::FillPortion(1))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    ];

    scrollable(table(columns, &tab.entries).padding_x(2).padding_y(4)).into()
}

pub fn view<'a>(_state: &'a AppState, tab: &'a HistoryTab) -> Element<'a, HistoryTabMsg> {
    let search_placeholder = if tab.is_searching {
        "Searching..."
    } else {
        "Search (method, URL, body, description)..."
    };

    let search_input = container(
        line_editor(&tab.search_query)
            .placeholder(search_placeholder)
            .highlight(false)
            .map(HistoryTabMsg::SearchChanged),
    )
    .width(Length::FillPortion(1))
    .padding(4);

    let is_empty = tab.search_query_text.is_empty();

    let clear_search_button = icon_button(icons::Close, Some(24), Some(10))
        .style(button::secondary)
        .on_press_maybe(is_empty.then_some(HistoryTabMsg::ClearSearch));

    let clear_history_button = icon_button(icons::Delete, Some(24), Some(10))
        .style(button::danger)
        .on_press(HistoryTabMsg::ClearHistory);

    let search_row = row![search_input, clear_search_button, clear_history_button]
        .height(Length::Shrink)
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .spacing(5);

    let content: Element<'a, HistoryTabMsg> = if let Some(error) = &tab.error {
        text(format!("Error: {error}")).into()
    } else if tab.entries.is_empty() {
        let message = if is_empty {
            "No entries found"
        } else {
            "No matching entries found"
        };
        text(message).into()
    } else {
        container(table_view(tab))
            .style(container::bordered_box)
            .into()
    };

    column![search_row, content]
        .spacing(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
