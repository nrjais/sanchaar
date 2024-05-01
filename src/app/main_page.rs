use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{scrollable, text, vertical_rule, Column, Row};
use iced::{Color, Command, Element, Font, Length};

use crate::app::panels::PanelMsg;

use crate::app::{collection_tree, panels};
use crate::state::{AppState, TabKey};
use components::{card_tab, card_tabs, TabBarAction};
use core::http::request::Method;

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    CollectionTree(CollectionTreeMsg),
}

impl MainPageMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            Self::TabBarAction(action) => {
                use TabBarAction::*;
                match action {
                    ChangeTab(tab) => state.active_tab = tab,
                    NewTab => state.active_tab = state.tabs.insert(Default::default()),
                    CloseTab(key) => state.close_tab(key),
                }
                Command::none()
            }
            Self::Panel(msg) => msg.update(state).map(Self::Panel),
            Self::CollectionTree(msg) => msg.update(state).map(Self::CollectionTree),
        }
    }
}

fn method_color(method: Method) -> Color {
    match method {
        Method::GET => Color::from_rgb8(0, 0, 255),
        Method::POST => Color::from_rgb8(0, 180, 0),
        Method::PUT => Color::from_rgb8(255, 165, 0),
        Method::DELETE => Color::from_rgb8(200, 0, 0),
        Method::PATCH => Color::from_rgb8(128, 0, 128),
        Method::HEAD => Color::from_rgb8(0, 0, 0),
        Method::OPTIONS => Color::from_rgb8(0, 128, 128),
        Method::CONNECT => Color::from_rgb8(255, 0, 255),
        Method::TRACE => Color::from_rgb8(150, 150, 150),
    }
}

pub fn view(state: &AppState) -> Element<MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(_, v)| v.id);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| {
            card_tab(
                key,
                text(tab.request.method.to_string())
                    .color(method_color(tab.request.method))
                    .shaping(Advanced)
                    .size(12)
                    .height(Length::Shrink)
                    .font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    }),
                text(
                    state
                        .col_req_ref(key)
                        .map(|a| &a.name as &str)
                        .unwrap_or("Untitled"),
                ),
            )
        })
        .collect();

    let content = Column::new()
        .push(card_tabs(
            state.active_tab,
            tabs,
            MainPageMsg::TabBarAction,
            None,
        ))
        .push(panels::view(state).map(MainPageMsg::Panel))
        .spacing(8)
        .align_items(iced::Alignment::Center)
        .width(Length::FillPortion(5));

    let tree = scrollable(collection_tree::view(state).map(MainPageMsg::CollectionTree))
        .height(Length::Fill)
        .width(Length::FillPortion(1));

    Row::new()
        .push(tree)
        .push(vertical_rule(4))
        .push(content)
        .spacing(4)
        .padding(4)
        .into()
}
