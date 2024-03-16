use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{scrollable, text, vertical_rule, Column, Row};
use iced::{Color, Element, Font, Length};

use crate::app::panels::PanelMsg;
use crate::app::{collection_tree, panels};
use crate::components::{card_tab, card_tabs, TabBarAction};
use crate::state::request::Method;
use crate::state::{AppState, TabKey};

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    CollectionTree(collection_tree::CollectionTreeMsg),
}

impl MainPageMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::TabBarAction(action) => match action {
                TabBarAction::ChangeTab(tab) => state.active_tab = tab,
                TabBarAction::NewTab => state.active_tab = state.tabs.insert(Default::default()),
                TabBarAction::CloseTab(key) => state.close_tab(key),
            },
            Self::Panel(msg) => msg.update(state),
            Self::CollectionTree(msg) => msg.update(state),
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
    tabs.sort_unstable_by_key(|(k, _v)| *k);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| {
            card_tab(
                key,
                text(tab.request.method)
                    .color(method_color(tab.request.method))
                    .shaping(Advanced)
                    .size(12)
                    .height(Length::Shrink)
                    .font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    }),
                text(&tab.request.name),
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
