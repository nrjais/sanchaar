use iced::font::Weight;
use iced::widget::text::Shaping::Advanced;
use iced::widget::{button, scrollable, text, vertical_rule, Button, Column, Row};
use iced::{theme, Color, Element, Font, Length};

use crate::components::{card_tab, card_tabs, icon, icons, NerdIcon, TabBarAction};
use crate::panels;
use crate::panels::PanelMsg;
use crate::state::collection::{Entry, Folder};
use crate::state::request::Method;
use crate::state::{AppState, CollectionKey, TabKey};

#[derive(Debug, Clone)]
pub enum MainPageMsg {
    TabBarAction(TabBarAction<TabKey>),
    Panel(PanelMsg),
    Test,
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, String),
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
            Self::ToggleExpandCollection(key) => {
                if let Some(collection) = state.collections.get_mut(key) {
                    collection.expanded = !collection.expanded;
                }
            }
            Self::ToggleFolder(col, name) => {
                if let Some(collection) = state.collections.get_mut(col) {
                    if let Some(folder) =
                        collection
                            .children
                            .iter_mut()
                            .find_map(|entry| match entry {
                                Entry::Folder(folder) if folder.name == name => Some(folder),
                                _ => None,
                            })
                    {
                        folder.expanded = !folder.expanded;
                    }
                }
            }
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
                    .style(theme::Text::Color(method_color(tab.request.method)))
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

    let tree = scrollable(collection_tree(state))
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

fn folder_tree(col: CollectionKey, entries: &[Entry], depth: u16) -> Element<MainPageMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => text(&item.name).into(),
        Entry::Folder(folder) => expandable(
            col,
            depth,
            &folder.name,
            &folder.children,
            folder.expanded,
            MainPageMsg::ToggleFolder(col, folder.name.clone()),
        ),
    });

    Column::with_children(it)
        .spacing(2)
        .padding([0, 0, 0, 8 * depth])
        .width(Length::Fill)
        .into()
}

fn expandable<'a>(
    col: CollectionKey,
    depth: u16,
    name: &str,
    entries: &'a [Entry],
    expanded: bool,
    on_expand_toggle: MainPageMsg,
) -> Element<'a, MainPageMsg> {
    let children = folder_tree(col, entries, depth + 1);
    if expanded {
        Column::new()
            .push(expandable_button(
                name,
                on_expand_toggle,
                icons::TriangleDown,
            ))
            .push(children)
            .spacing(2)
            .width(Length::Fill)
            .into()
    } else {
        expandable_button(name, on_expand_toggle, icons::TriangleRight).into()
    }
}

fn expandable_button<'a>(
    name: &str,
    on_expand_toggle: MainPageMsg,
    arrow: NerdIcon,
) -> Button<'a, MainPageMsg> {
    button(
        Row::with_children([icon(arrow).size(12).into(), text(name).into()])
            .align_items(iced::Alignment::Center)
            .spacing(4),
    )
    .style(theme::Button::Text)
    .padding(0)
    .on_press(on_expand_toggle)
    .width(Length::Fill)
}

fn collection_tree(state: &AppState) -> Element<MainPageMsg> {
    let it = state.collections.iter().map(|(key, collection)| {
        expandable(
            key,
            0,
            &collection.name,
            &collection.children,
            collection.expanded,
            MainPageMsg::ToggleExpandCollection(key),
        )
    });

    Column::with_children(it)
        .spacing(4)
        .width(Length::Fill)
        .into()
}
