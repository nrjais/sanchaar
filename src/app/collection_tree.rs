use crate::commands::AppCommand;
use iced::widget::{button, text, Button, Column, Row};
use iced::{Element, Length};

use crate::components::{icon, icons, NerdIcon};
use crate::state::collection::{Entry, Item};
use crate::state::{AppState, CollectionKey};

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, String),
    OpenRequest(CollectionKey, Item),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::ToggleExpandCollection(key) => {
                state.with_collection(key, |collection| collection.toggle_expand());
            }
            Self::ToggleFolder(col, name) => {
                state.with_collection(col, |collection| collection.toggle_folder(&name));
            }
            CollectionTreeMsg::OpenRequest(col, req) => {
                state.commands.add(AppCommand::OpenRequest(col, req));
            }
        }
    }
}

pub fn view(state: &AppState) -> Element<CollectionTreeMsg> {
    let it = state.collections.iter().map(|(key, collection)| {
        expandable(
            key,
            &collection.name,
            &collection.children,
            collection.expanded,
            CollectionTreeMsg::ToggleExpandCollection(key),
        )
    });

    Column::with_children(it)
        .spacing(4)
        .width(Length::Fill)
        .into()
}

fn folder_tree(col: CollectionKey, entries: &[Entry]) -> Element<CollectionTreeMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => button(text(&item.name))
            .style(button::text)
            .padding(0)
            .on_press(CollectionTreeMsg::OpenRequest(col, item.clone()))
            .into(),
        Entry::Folder(folder) => expandable(
            col,
            &folder.name,
            &folder.children,
            folder.expanded,
            CollectionTreeMsg::ToggleFolder(col, folder.name.clone()),
        ),
    });

    Column::with_children(it)
        .spacing(2)
        .padding([0, 0, 0, 12])
        .width(Length::Fill)
        .into()
}

fn expandable<'a>(
    col: CollectionKey,
    name: &'a str,
    entries: &'a [Entry],
    expanded: bool,
    on_expand_toggle: CollectionTreeMsg,
) -> Element<'a, CollectionTreeMsg> {
    if expanded {
        let children = folder_tree(col, entries);
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

fn expandable_button(
    name: &str,
    on_expand_toggle: CollectionTreeMsg,
    arrow: NerdIcon,
) -> Button<CollectionTreeMsg> {
    button(
        Row::with_children([icon(arrow).size(12).into(), text(name).into()])
            .align_items(iced::Alignment::Center)
            .spacing(4),
    )
    .style(button::text)
    .padding(0)
    .on_press(on_expand_toggle)
    .width(Length::Fill)
}
