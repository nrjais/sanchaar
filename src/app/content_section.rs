use std::time::Duration;

use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::{Column, container, text};
use iced::{Alignment, Color, Element, Font, Length, Task, padding};

use crate::app::bottom_bar::BottomBarMsg;
use crate::app::panels::PanelMsg;

use crate::app::{bottom_bar, collection_tree, panels};
use crate::components::split::vertical_split;
use crate::components::{CardTab, TabBarAction, card_tab, card_tabs, colors, icon, icons};
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::tabs::history_tab::HistoryTab;
use crate::state::tabs::perf_tab::PerfTab;
use crate::state::{AppState, HttpTab, Tab, TabKey};
use lib::http::request::Method;

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    CollectionTree(CollectionTreeMsg),
    SplitResize(f32),
    BottomBar(BottomBarMsg),
    OpenHistoryTab,
}

impl MainPageMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            Self::TabBarAction(action) => {
                use TabBarAction::*;
                match action {
                    ChangeTab(tab) => state.switch_tab(tab),
                    NewTab => state.open_tab(Tab::Http(HttpTab::new_def())),
                    CloseTab(key) => state.close_tab(key),
                    TabDrop(point, _, dragged_tab) => {
                        return iced_drop::zones_on_point(
                            move |zones| Self::TabBarAction(HandleDropZones(zones, dragged_tab)),
                            point,
                            None,
                            None,
                        );
                    }
                    HandleDropZones(zones, dragged_tab) => {
                        let target_tab_keys: Vec<_> = state.tabs.keys().copied().collect();
                        for (zone_id, _) in zones {
                            for target_key in &target_tab_keys {
                                let expected_zone_id = format!("tab-{}", target_key).into();
                                if zone_id == expected_zone_id {
                                    state.reorder_tab(dragged_tab, *target_key);
                                    return Task::none();
                                }
                            }
                        }
                    }
                }
                Task::none()
            }
            Self::Panel(msg) => msg.update(state).map(Self::Panel),
            Self::CollectionTree(msg) => msg.update(state).map(Self::CollectionTree),
            Self::SplitResize(ratio) => {
                state.pane_config.set_at(ratio);
                Task::none()
            }
            Self::OpenHistoryTab => {
                let existing_tab = state
                    .tabs
                    .iter()
                    .find(|(_, tab)| matches!(tab, Tab::History(_)))
                    .map(|(key, _)| *key);

                if let Some(tab) = existing_tab {
                    state.switch_tab(tab);
                } else {
                    state.open_tab(Tab::History(HistoryTab::new()));
                }
                Task::none()
            }
            Self::BottomBar(msg) => msg.update(state).map(Self::BottomBar),
        }
    }
}

fn method_color(_method: Method) -> Color {
    colors::CYAN
}

fn split_content(state: &AppState) -> Element<MainPageMsg> {
    let tab_content = tab_panel(state);
    let pane_config = &state.pane_config;
    if pane_config.side_bar_open {
        vertical_split(
            side_bar(state),
            tab_content,
            pane_config.at,
            MainPageMsg::SplitResize,
        )
        .handle_width(8.)
        .focus_delay(Duration::from_millis(50))
        .into()
    } else {
        tab_content
    }
}

pub fn view(state: &AppState) -> Element<MainPageMsg> {
    Column::new()
        .push(split_content(state))
        .push(bottom_bar::view(state).map(MainPageMsg::BottomBar))
        .into()
}

fn side_bar(state: &AppState) -> Element<MainPageMsg> {
    collection_tree::view(state).map(MainPageMsg::CollectionTree)
}

fn tab_panel(state: &AppState) -> Element<MainPageMsg> {
    match state.active_tab() {
        Some(tab) => tabs_view(state, state.active_tab, tab),
        None => no_tabs_view(),
    }
}

fn no_tabs_view<'a>() -> Element<'a, MainPageMsg> {
    container(
        Column::new()
            .push(container(icon(icons::FolderOpen).size(80.0)).padding(10))
            .push(iced::widget::Text::new("No tabs open").size(20))
            .align_x(Alignment::Center),
    )
    .center(Length::Fill)
    .into()
}

fn tabs_view<'a>(
    state: &'a AppState,
    active_tab: TabKey,
    tab: &'a Tab,
) -> Element<'a, MainPageMsg> {
    let tabs = state
        .tabs
        .iter()
        .map(|(key, tab)| match tab {
            Tab::Http(tab) => tab_card(*key, tab),
            Tab::Collection(tab) => col_tab(*key, tab),
            Tab::CookieStore(_) => cookie_tab(*key),
            Tab::History(tab) => history_tab(*key, tab),
            Tab::Perf(tab) => perf_tab(*key, tab),
        })
        .collect();

    let tabs = Column::new()
        .push(card_tabs(active_tab, tabs, MainPageMsg::TabBarAction, None))
        .push(panels::view(state, tab).map(MainPageMsg::Panel))
        .spacing(8)
        .align_x(Alignment::Center);

    container(tabs)
        .padding(padding::left(4).right(4).top(4))
        .into()
}

fn col_tab(key: TabKey, tab: &CollectionTab) -> CardTab<TabKey> {
    card_tab(key, icon(icons::Folder), text(&tab.name))
}

fn cookie_tab<'a>(key: TabKey) -> CardTab<'a, TabKey> {
    card_tab(key, icon(icons::Cookie), text("Cookies"))
}

fn history_tab<'a>(key: TabKey, tab: &'a HistoryTab) -> CardTab<'a, TabKey> {
    card_tab(key, icon(icons::History), text(&tab.name))
}

fn perf_tab<'a>(key: TabKey, _tab: &'a PerfTab) -> CardTab<'a, TabKey> {
    card_tab(key, icon(icons::Speedometer), text("Performance"))
}

fn tab_card<'a>(key: TabKey, tab: &'a HttpTab) -> CardTab<'a, TabKey> {
    let dirty_flag = if tab.is_request_dirty() { "" } else { "" };
    card_tab(
        key,
        text(format!("{}{}", dirty_flag, tab.request().method))
            .color(method_color(tab.request().method))
            .size(12)
            .height(Length::Shrink)
            .font(Font {
                weight: Weight::Bold,
                ..Default::default()
            }),
        text(&tab.name),
    )
}
