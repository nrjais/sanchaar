use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{container, pane_grid, text, Column, PaneGrid};
use iced::{Color, Task, Element, Font, Length};

use crate::app::panels::PanelMsg;

use crate::app::{collection_tree, panels};
use crate::state::{AppState, SplitState, TabKey};
use components::{bordered_left, bordered_right, card_tab, card_tabs, colors, TabBarAction};
use core::http::request::Method;

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
                    ChangeTab(tab) => state.active_tab = tab,
                    NewTab => state.active_tab = state.tabs.insert(Default::default()),
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
            SplitState::Second => tabs_view(state),
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
            .padding([0, 4, 0, 0]),
    )
}

fn tabs_view(state: &AppState) -> Element<MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(_, v)| v.id);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| {
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
                text(
                    state
                        .get_req_ref(key)
                        .map(|a| &a.name as &str)
                        .unwrap_or("Untitled"),
                ),
            )
        })
        .collect();

    let tabs = Column::new()
        .push(card_tabs(
            state.active_tab,
            tabs,
            MainPageMsg::TabBarAction,
            None,
        ))
        .push(panels::view(state).map(MainPageMsg::Panel))
        .spacing(8)
        .align_items(iced::Alignment::Center);

    bordered_left(BORDER_WIDTH, container(tabs).padding([0, 0, 0, 4]))
}
