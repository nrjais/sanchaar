use iced::{Command, Element, Length};
use iced::widget::{button, Button, Column, container, horizontal_rule, Row, text, tooltip};
use iced::widget::tooltip::Position;

use components::{icon, icons, NerdIcon};
use core::http::{CollectionKey, CollectionRequest};
use core::http::collection::{Collection, Entry, FolderId};

use crate::commands::AppCommand;
use crate::commands::builders::open_existing_collection;
use crate::state::AppState;
use crate::state::popups::Popup;

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, FolderId),
    OpenRequest(CollectionRequest),
    CreateCollection,
    OpenCollection,
    OpenCollectionHandle(Option<Collection>),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
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
            Self::OpenRequest(col) => {
                if !state.switch_to_tab(col) {
                    state.commands.add(AppCommand::OpenRequest(col));
                };
            }
            Self::CreateCollection => {
                state.popup = Some(Popup::CreateCollection(Default::default()));
            }
            Self::OpenCollection => {
                return open_existing_collection(Self::OpenCollectionHandle);
            }
            Self::OpenCollectionHandle(handle) => {
                if let Some(handle) = handle {
                    state.collections.insert(handle);
                }
            }
        }
        Command::none()
    }
}
fn icon_button<'a>(ico: NerdIcon) -> Button<'a, CollectionTreeMsg> {
    button(container(icon(ico).size(20)).padding([0, 8]))
        .padding(0)
        .style(button::secondary)
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

    let create_col = tooltip(
        icon_button(icons::Plus).on_press(CollectionTreeMsg::CreateCollection),
        container(text("Create Collection"))
            .padding([2, 4])
            .style(container::rounded_box),
        Position::Bottom,
    );

    let open_col = tooltip(
        icon_button(icons::FolderOpen).on_press(CollectionTreeMsg::CreateCollection),
        container(text("Open Collection"))
            .padding([2, 4])
            .style(container::rounded_box),
        Position::Bottom,
    );

    Column::new()
        .push(
            Row::new()
                .push(create_col)
                .push(open_col)
                .width(Length::Fill)
                .spacing(4),
        )
        .push(horizontal_rule(4))
        .push(Column::with_children(it).spacing(4))
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
