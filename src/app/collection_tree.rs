use iced::alignment::Horizontal;
use iced::widget::tooltip::Position;
use iced::widget::{
    button, column, container, horizontal_rule, row, text, tooltip, Button, Column, Row,
};
use iced::{Command, Element, Length};

use components::{context_menu, icon, icons, menu_item, NerdIcon};
use core::http::collection::{Collection, Entry, FolderId};
use core::http::{request::Request, CollectionKey, CollectionRequest};

use crate::commands::builders::{self, open_existing_collection, open_request_cmd};
use crate::state::popups::Popup;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, FolderId),
    OpenRequest(CollectionRequest),
    CreateCollection,
    OpenCollection,
    OpenCollectionHandle(Option<Collection>),
    RequestLoaded(CollectionRequest, Option<Request>),
    ContextMenu(CollectionKey, MenuAction),
    ActionComplete(MenuAction),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CollectionTreeMsg::ToggleExpandCollection(key) => {
                state
                    .collections
                    .with_collection_mut(key, |collection| collection.toggle_expand());
            }
            CollectionTreeMsg::ToggleFolder(col, id) => {
                state
                    .collections
                    .with_collection_mut(col, |collection| collection.toggle_folder(id));
            }
            CollectionTreeMsg::OpenRequest(col) => {
                if !state.switch_to_tab(col) {
                    return open_request_cmd(state, col, move |res| Self::RequestLoaded(col, res));
                };
            }
            CollectionTreeMsg::CreateCollection => {
                state.popup = Some(Popup::CreateCollection(Default::default()));
            }
            CollectionTreeMsg::OpenCollection => {
                return open_existing_collection(Self::OpenCollectionHandle);
            }
            CollectionTreeMsg::OpenCollectionHandle(handle) => {
                if let Some(handle) = handle {
                    state.collections.insert(handle);
                }
            }
            CollectionTreeMsg::RequestLoaded(col, req) => {
                if let Some(req) = req {
                    state.open_request(col, req);
                }
            }
            CollectionTreeMsg::ContextMenu(col, action) => {
                return handle_context_menu(state, col, action);
            }
            CollectionTreeMsg::ActionComplete(_) => {}
        }
        Command::none()
    }
}

fn handle_context_menu(
    state: &mut AppState,
    col: CollectionKey,
    action: MenuAction,
) -> Command<CollectionTreeMsg> {
    match action {
        MenuAction::NewFolderRoot => {
            state.popup = Some(Popup::create_folder(col, None));
            Command::none()
        }
        MenuAction::NewFolder(folder_id) => {
            state.popup = Some(Popup::create_folder(col, Some(folder_id)));
            Command::none()
        }
        MenuAction::DeleteFolder(folder_id) => {
            builders::delete_folder_cmd(state, col, folder_id, move || {
                CollectionTreeMsg::ActionComplete(action)
            })
        }
        MenuAction::RemoveCollection => {
            state.collections.remove(col);
            Command::none()
        }
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
            None,
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
        icon_button(icons::FolderOpen).on_press(CollectionTreeMsg::OpenCollection),
        container(text("Open Collection"))
            .padding([2, 4])
            .style(container::rounded_box),
        Position::Bottom,
    );

    Column::new()
        .push(
            container(
                Row::new()
                    .push(create_col)
                    .push(open_col)
                    .width(Length::Shrink)
                    .spacing(4),
            )
            .align_x(Horizontal::Center)
            .width(Length::Fill),
        )
        .push(horizontal_rule(4))
        .push(column(it).spacing(4))
        .spacing(4)
        .width(Length::Fill)
        .into()
}

fn folder_tree(col: CollectionKey, entries: &[Entry]) -> Element<CollectionTreeMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => button(
            row([icon(icons::API).into(), text(&item.name).into()])
                .align_items(iced::Alignment::Center)
                .spacing(6),
        )
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
            Some(folder.id),
        ),
    });

    column(it)
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
    folder_id: Option<FolderId>,
) -> Element<'a, CollectionTreeMsg> {
    if expanded {
        let children = folder_tree(col, entries);
        Column::new()
            .push(expandable_button(
                name,
                on_expand_toggle,
                icons::TriangleDown,
                col,
                folder_id,
            ))
            .push(children)
            .spacing(2)
            .width(Length::Fill)
            .into()
    } else {
        expandable_button(name, on_expand_toggle, icons::TriangleRight, col, folder_id).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MenuAction {
    NewFolder(FolderId),
    DeleteFolder(FolderId),
    NewFolderRoot,
    RemoveCollection,
}

fn context_button_folder<'a>(
    base: impl Into<Element<'a, CollectionTreeMsg>>,
    col: CollectionKey,
    folder_id: FolderId,
) -> Element<'a, CollectionTreeMsg> {
    context_menu(
        base,
        vec![
            menu_item(
                "New Folder",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewFolder(folder_id)),
            ),
            menu_item(
                "Delete Folder",
                CollectionTreeMsg::ContextMenu(col, MenuAction::DeleteFolder(folder_id)),
            ),
        ],
    )
}

fn context_button_collection<'a>(
    base: impl Into<Element<'a, CollectionTreeMsg>>,
    col: CollectionKey,
) -> Element<'a, CollectionTreeMsg> {
    context_menu(
        base,
        vec![
            menu_item(
                "Remove",
                CollectionTreeMsg::ContextMenu(col, MenuAction::RemoveCollection),
            ),
            menu_item(
                "New Folder",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewFolderRoot),
            ),
        ],
    )
}

fn expandable_button(
    name: &str,
    on_expand_toggle: CollectionTreeMsg,
    arrow: NerdIcon,
    col: CollectionKey,
    folder_id: Option<FolderId>,
) -> impl Into<Element<CollectionTreeMsg>> {
    let base = button(
        row([icon(arrow).into(), text(name).into()])
            .align_items(iced::Alignment::Center)
            .spacing(4),
    )
    .style(button::text)
    .on_press(on_expand_toggle)
    .width(Length::Fill)
    .padding(0);

    if let Some(folder_id) = folder_id {
        context_button_folder(base, col, folder_id)
    } else {
        context_button_collection(base, col)
    }
}
