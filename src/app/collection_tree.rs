use iced::alignment::Horizontal;
use iced::widget::scrollable::Direction;
use iced::widget::tooltip::Position;
use iced::widget::{
    button, column, container, row, text, tooltip, Button, Column, Row, Scrollable,
};
use iced::{Command, Element, Length};

use components::{context_menu, horizontal_line, icon, icons, menu_item, NerdIcon};
use core::http::collection::{Collection, Entry, FolderId, RequestId, RequestRef};
use core::http::{request::Request, CollectionKey, CollectionRequest};

use crate::commands::builders::{self, open_collection_cmd, open_request_cmd};
use crate::state::popups::{Popup, PopupNameAction};
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
    ActionComplete,
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
                Popup::create_collection(state);
            }
            CollectionTreeMsg::OpenCollection => {
                return open_collection_cmd(Self::OpenCollectionHandle);
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
            CollectionTreeMsg::ActionComplete => {}
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
        MenuAction::NewRequest(folder_id) => {
            Popup::popup_name(
                state,
                String::new(),
                PopupNameAction::NewRequest(col, folder_id),
            );
            Command::none()
        }
        MenuAction::NewFolder(folder_id) => {
            Popup::popup_name(
                state,
                String::new(),
                PopupNameAction::CreateFolder(col, folder_id),
            );
            Command::none()
        }
        MenuAction::DeleteFolder(folder_id) => {
            builders::delete_folder_cmd(state, col, folder_id, move || {
                CollectionTreeMsg::ActionComplete
            })
        }
        MenuAction::RemoveCollection => {
            state.collections.remove(col);
            Command::none()
        }
        MenuAction::RenameFolder(name, folder_id) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameFolder(col, folder_id),
            );
            Command::none()
        }
        MenuAction::RenameCollection(name) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameCollection(col),
            );
            Command::none()
        }
        MenuAction::RenameRequest(name, req) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameRequest(col, req),
            );
            Command::none()
        }
        MenuAction::DeleteRequest(req) => {
            builders::delete_request_cmd(state, col, req, move || CollectionTreeMsg::ActionComplete)
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, CollectionTreeMsg> {
    button(container(icon(ico).size(20)).padding([0, 8]))
        .padding(0)
        .style(button::secondary)
        .width(Length::Shrink)
}

pub fn view(state: &AppState) -> Element<CollectionTreeMsg> {
    let it = state.collections.iter().map(|(key, collection)| {
        expandable(
            key,
            &collection.name,
            &collection.entries,
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
        .push(horizontal_line(2))
        .push(
            Scrollable::with_direction(
                column(it)
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .spacing(4)
                    .padding([0, 12, 12, 0]),
                Direction::Both {
                    vertical: Default::default(),
                    horizontal: Default::default(),
                },
            )
            .height(Length::Fill),
        )
        .spacing(4)
        .width(Length::Fill)
        .into()
}

fn folder_tree(col: CollectionKey, entries: &[Entry]) -> Element<CollectionTreeMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => context_button_request(item, col),
        Entry::Folder(folder) => expandable(
            col,
            &folder.name,
            &folder.entries,
            folder.expanded,
            CollectionTreeMsg::ToggleFolder(col, folder.id),
            Some(folder.id),
        ),
    });

    column(it)
        .spacing(2)
        .padding([0, 0, 0, 12])
        .width(Length::Shrink)
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
                icons::FolderOpen,
                col,
                folder_id,
            ))
            .push(children)
            .spacing(2)
            .width(Length::Shrink)
            .into()
    } else {
        expandable_button(name, on_expand_toggle, icons::Folder, col, folder_id).into()
    }
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
            .width(Length::Shrink)
            .spacing(4),
    )
    .style(button::text)
    .on_press(on_expand_toggle)
    .width(Length::Shrink)
    .padding(0);

    if let Some(folder_id) = folder_id {
        context_button_folder(base, name.to_owned(), col, folder_id)
    } else {
        context_button_collection(base, name.to_owned(), col)
    }
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    NewFolder(Option<FolderId>),
    RenameFolder(String, FolderId),
    DeleteFolder(FolderId),
    RenameRequest(String, RequestId),
    DeleteRequest(RequestId),
    NewRequest(Option<FolderId>),
    RenameCollection(String),
    RemoveCollection,
}

fn context_button_folder<'a>(
    base: impl Into<Element<'a, CollectionTreeMsg>>,
    name: String,
    col: CollectionKey,
    folder_id: FolderId,
) -> Element<'a, CollectionTreeMsg> {
    context_menu(
        base,
        vec![
            menu_item(
                "Rename",
                CollectionTreeMsg::ContextMenu(col, MenuAction::RenameFolder(name, folder_id)),
            ),
            menu_item(
                "New Request",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewRequest(Some(folder_id))),
            ),
            menu_item(
                "New Folder",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewFolder(Some(folder_id))),
            ),
            menu_item(
                "Delete",
                CollectionTreeMsg::ContextMenu(col, MenuAction::DeleteFolder(folder_id)),
            ),
        ],
    )
}

fn context_button_request<'a>(
    item: &'a RequestRef,
    col: CollectionKey,
) -> Element<'a, CollectionTreeMsg> {
    let collection_request = CollectionRequest(col, item.id);

    let base = button(
        row([
            icon(icons::API)
                .style(|t| text::Style {
                    color: Some(t.extended_palette().success.strong.color),
                })
                .into(),
            text(&item.name).into(),
        ])
        .align_items(iced::Alignment::Center)
        .width(Length::Shrink)
        .spacing(6),
    )
    .style(button::text)
    .padding(0)
    .width(Length::Shrink)
    .on_press(CollectionTreeMsg::OpenRequest(collection_request));

    let request_id = item.id;
    context_menu(
        base,
        vec![
            menu_item(
                "Rename",
                CollectionTreeMsg::ContextMenu(
                    col,
                    MenuAction::RenameRequest(item.name.to_owned(), request_id),
                ),
            ),
            menu_item(
                "Delete",
                CollectionTreeMsg::ContextMenu(col, MenuAction::DeleteRequest(request_id)),
            ),
        ],
    )
    .into()
}

fn context_button_collection<'a>(
    base: impl Into<Element<'a, CollectionTreeMsg>>,
    name: String,
    col: CollectionKey,
) -> Element<'a, CollectionTreeMsg> {
    context_menu(
        base,
        vec![
            menu_item(
                "Rename",
                CollectionTreeMsg::ContextMenu(col, MenuAction::RenameCollection(name)),
            ),
            menu_item(
                "New Request",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewRequest(None)),
            ),
            menu_item(
                "New Folder",
                CollectionTreeMsg::ContextMenu(col, MenuAction::NewFolder(None)),
            ),
            menu_item(
                "Remove",
                CollectionTreeMsg::ContextMenu(col, MenuAction::RemoveCollection),
            ),
        ],
    )
}
