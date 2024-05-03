use collection_tree::CollectionTreeMsg;
use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{scrollable, text, vertical_rule, Column, Row};
use iced::{Color, Command, Element, Font, Length};

use crate::app::panels::PanelMsg;

use crate::app::{collection_tree, panels};
use crate::state::{AppState, TabKey};
use components::{card_tab, card_tabs, colors, TabBarAction};
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

fn method_color(_method: Method) -> Color {
    colors::CYAN
    // Color::from_rgb8(0, 0, 0)
}

pub fn view(state: &AppState) -> Element<MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(_, v)| v.id);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| {
            let dirty_flag = tab.is_request_dirty().then_some("").unwrap_or("");
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
