use crate::commands::AppCommand;
use iced::widget::{button, text, Button, Column, Row};
use iced::{Element, Length};

use crate::state::AppState;
use components::{icon, icons, NerdIcon};
use core::collection::collection::{Entry, FolderId};
use core::collection::{CollectionKey, CollectionRequest};

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, FolderId),
    OpenRequest(CollectionRequest),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::ToggleExpandCollection(key) => {
                state
                    .collections
                    .on_collection_mut(key, |collection| collection.toggle_expand());
            }
            Self::ToggleFolder(col, id) => {
                state
                    .collections
                    .on_collection_mut(col, |collection| collection.toggle_folder(id));
            }
            CollectionTreeMsg::OpenRequest(col) => {
                if !state.switch_to_tab(col) {
                    state.commands.add(AppCommand::OpenRequest(col));
                };
            }
        }
    }
}

pub fn view(state: &AppState) -> Element<CollectionTreeMsg> {
    let it = state.collections.entries.iter().map(|(key, collection)| {
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
            .on_press(CollectionTreeMsg::OpenRequest(CollectionRequest(
                col, item.id,
            )))
            .into(),
        Entry::Folder(folder) => expandable(
            col,
            &folder.name,
            &folder.children,
            folder.expanded,
            CollectionTreeMsg::ToggleFolder(col, folder.id),
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
