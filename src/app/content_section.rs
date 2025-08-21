use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{Column, PaneGrid, container, pane_grid, text};
use iced::{Color, Element, Font, Length, Task, padding};

use crate::app::panels::PanelMsg;

use crate::app::{collection_tree, panels};
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::tabs::history_tab::HistoryTab;
use crate::state::{AppState, HttpTab, SplitState, Tab, TabKey};
use components::{
    CardTab, TabBarAction, bordered_left, bordered_right, card_tab, card_tabs, colors, icon, icons,
};
use core::http::request::Method;

const BORDER_WIDTH: u16 = 1;

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    CollectionTree(CollectionTreeMsg),
    SplitResize(ResizeEvent),
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
                }
                Task::none()
            }
            Self::Panel(msg) => msg.update(state).map(Self::Panel),
            Self::CollectionTree(msg) => msg.update(state).map(Self::CollectionTree),
            Self::SplitResize(ResizeEvent { split, ratio }) => {
                if ratio > 0.20 && ratio < 0.35 {
                    state.panes.resize(split, ratio);
                }
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
        }
    }
}

fn method_color(_method: Method) -> Color {
    colors::CYAN
    // Color::from_rgb8(0, 0, 0)
}

pub fn view(state: &AppState) -> Element<MainPageMsg> {
    let panes = PaneGrid::new(&state.panes, move |_, pane, _| {
        let pane = match pane {
            SplitState::First => side_bar(state),
            SplitState::Second => tab_panel(state),
        };
        pane_grid::Content::new(pane)
    })
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .on_resize(8, MainPageMsg::SplitResize);

    container(panes).padding(8).into()
}

fn side_bar(state: &AppState) -> Element<MainPageMsg> {
    bordered_right(
        BORDER_WIDTH,
        container(collection_tree::view(state).map(MainPageMsg::CollectionTree))
            .padding(padding::right(4)),
    )
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
            .align_x(iced::Alignment::Center),
    )
    .center(Length::Fill)
    .into()
}

fn tabs_view<'a>(
    state: &'a AppState,
    active_tab: TabKey,
    tab: &'a Tab,
) -> Element<'a, MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(key, _)| *key);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| match tab {
            Tab::Http(tab) => tab_card(*key, tab),
            Tab::Collection(tab) => col_tab(*key, tab),
            Tab::CookieStore(_) => cookie_tab(*key),
            Tab::History(tab) => history_tab(*key, tab),
        })
        .collect();

    let tabs = Column::new()
        .push(card_tabs(active_tab, tabs, MainPageMsg::TabBarAction, None))
        .push(panels::view(state, tab).map(MainPageMsg::Panel))
        .spacing(8)
        .align_x(iced::Alignment::Center);

    bordered_left(BORDER_WIDTH, container(tabs).padding(padding::left(4)))
}

fn col_tab(key: TabKey, tab: &CollectionTab) -> CardTab<TabKey> {
    card_tab(key, icon(icons::Folder), text(&tab.name))
}

fn cookie_tab<'a>(key: TabKey) -> CardTab<'a, TabKey> {
    card_tab(key, icon(icons::Cookie), text("Cookies"))
}

fn history_tab<'a>(key: TabKey, tab: &'a HistoryTab) -> CardTab<'a, TabKey> {
    card_tab(key, icon(icons::CheckBold), text(&tab.name))
}

fn tab_card<'a>(key: TabKey, tab: &'a HttpTab) -> CardTab<'a, TabKey> {
    let dirty_flag = if tab.is_request_dirty() { "" } else { "" };
    card_tab(
        key,
        text(format!("{}{}", dirty_flag, tab.request().method))
            .color(method_color(tab.request().method))
            .shaping(Advanced)
            .size(12)
            .height(Length::Shrink)
            .font(Font {
                weight: Weight::Bold,
                ..Default::default()
            }),
        text(&tab.name),
    )
}
