use crate::components::{icon, icons, NerdIcon};
use crate::state::collection::Entry;
use crate::state::{AppState, CollectionKey};
use iced::widget::{button, text, Button, Column, Row};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, String),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::ToggleExpandCollection(key) => {
                if let Some(collection) = state.collections.get_mut(key) {
                    collection.toggle_expand();
                }
            }
            Self::ToggleFolder(col, name) => {
                if let Some(collection) = state.collections.get_mut(col) {
                    collection.toggle_folder(&name);
                }
            }
        }
    }
}

pub fn view(state: &AppState) -> Element<CollectionTreeMsg> {
    let it = state.collections.iter().map(|(key, collection)| {
        expandable(
            key,
            0,
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

fn folder_tree(col: CollectionKey, entries: &[Entry], depth: u16) -> Element<CollectionTreeMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => text(&item.name).into(),
        Entry::Folder(folder) => expandable(
            col,
            depth,
            &folder.name,
            &folder.children,
            folder.expanded,
            CollectionTreeMsg::ToggleFolder(col, folder.name.clone()),
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
    on_expand_toggle: CollectionTreeMsg,
) -> Element<'a, CollectionTreeMsg> {
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
    on_expand_toggle: CollectionTreeMsg,
    arrow: NerdIcon,
) -> Button<'a, CollectionTreeMsg> {
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
