use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{container, pane_grid, text, Column, PaneGrid};
use iced::{padding, Color, Element, Font, Length, Task};

use crate::app::panels::PanelMsg;

use crate::app::{collection_tree, panels};
use crate::state::{AppState, HttpTab, SplitState, Tab, TabKey};
use components::{
    bordered_left, bordered_right, card_tab, card_tabs, colors, icon, icons, CardTab, TabBarAction,
};
use core::http::request::Method;
use core::http::CollectionRequest;

const BORDER_WIDTH: u16 = 1;

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    CollectionTree(CollectionTreeMsg),
    SplitResize(ResizeEvent),
}

impl MainPageMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            Self::TabBarAction(action) => {
                use TabBarAction::*;
                match action {
                    ChangeTab(tab) => state.switch_tab(tab),
                    NewTab => state.open_new_tab(Tab::Http(HttpTab::new(
                        "Untitled".to_string(),
                        Default::default(),
                        CollectionRequest(Default::default(), Default::default()),
                    ))),
                    CloseTab(key) => state.close_tab(key),
                }
                Task::none()
            }
            Self::Panel(msg) => msg.update(state).map(Self::Panel),
            Self::CollectionTree(msg) => msg.update(state).map(Self::CollectionTree),
            Self::SplitResize(ResizeEvent { split, ratio }) => {
                // Only allow resizing if the ratio is min 0.15 and max 0.3
                if ratio > 0.1 && ratio < 0.3 {
                    state.panes.resize(split, ratio);
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
    match state.active_tab.zip(state.active_tab()) {
        Some((key, tab)) => tabs_view(state, key, tab),
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
            Tab::Http(tab) => tab_card(key, tab),
            Tab::Collection(_) => todo!(),
        })
        .collect();

    let tabs = Column::new()
        .push(card_tabs(active_tab, tabs, MainPageMsg::TabBarAction, None))
        .push(panels::view(state, tab).map(MainPageMsg::Panel))
        .spacing(8)
        .align_x(iced::Alignment::Center);

    bordered_left(BORDER_WIDTH, container(tabs).padding(padding::left(4)))
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
