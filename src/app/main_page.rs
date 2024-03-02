use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{button, scrollable, text, vertical_rule, Column, Row};
use iced::{theme, Color, Font, Length};

use crate::components::{card_tab, card_tabs, icon, icons, TabBarAction};
use crate::panels;
use crate::panels::PanelMsg;
use crate::state::request::Method;
use crate::state::{AppState, TabKey};

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    Test,
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
            Self::Test => println!("Test"),
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

pub fn view(state: &AppState) -> iced::Element<MainPageMsg> {
    let mut tabs = state.tabs.iter().collect::<Vec<_>>();
    tabs.sort_unstable_by_key(|(k, _v)| *k);

    let tabs = tabs
        .into_iter()
        .map(|(key, tab)| {
            card_tab(
                key,
                text(tab.request.method)
                    .style(iced::theme::Text::Color(method_color(tab.request.method)))
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

    let coll = Column::new()
        .push(
            button(Row::with_children([
                icon(icons::TriangleRight).into(),
                text("Collection 1").into(),
            ]))
            .style(theme::Button::Text)
            .padding(2)
            .width(Length::Fill),
        )
        .push(text("Collection 3"))
        .spacing(4)
        .width(Length::Fill);

    let tree = scrollable(coll)
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
