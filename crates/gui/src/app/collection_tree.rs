use iced::alignment::Horizontal;
use iced::widget::scrollable::Direction;
use iced::widget::{button, column, container, row, text, Button, Column, Row, Scrollable};
use iced::{clipboard, padding, Element, Length, Task};

use components::{context_menu, horizontal_line, icon, icons, menu_item, tooltip, NerdIcon};
use core::http::collection::{Collection, Entry, FolderId, RequestId, RequestRef};
use core::http::{request::Request, CollectionKey, CollectionRequest};

use crate::commands::builders::{self, open_collection_cmd, open_request_cmd};
use crate::state::collection_tab::CollectionTab;
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::{AppState, HttpTab, Tab};

#[derive(Debug, Clone)]
pub enum CollectionTreeMsg {
    ToggleExpandCollection(CollectionKey),
    ToggleFolder(CollectionKey, FolderId),
    OpenRequest(CollectionRequest),
    CreateCollection,
    OpenCollection,
    OpenCollectionHandle(Option<Collection>),
    RequestLoaded(CollectionRequest, Box<Option<(Request, String)>>),
    ContextMenu(CollectionKey, MenuAction),
    ActionComplete,
    OpenSettings,
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
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
                    return open_request_cmd(state, col, move |res| {
                        Self::RequestLoaded(col, Box::new(res))
                    });
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
                if let Some((req, name)) = *req {
                    state.open_tab(Tab::Http(HttpTab::new(name, req, col)));
                }
            }
            CollectionTreeMsg::ContextMenu(col, action) => {
                return handle_context_menu(state, col, action);
            }
            CollectionTreeMsg::ActionComplete => (),
            CollectionTreeMsg::OpenSettings => {
                Popup::app_settings(state);
            }
        };
        Task::none()
    }
}

fn handle_context_menu(
    state: &mut AppState,
    key: CollectionKey,
    action: MenuAction,
) -> Task<CollectionTreeMsg> {
    match action {
        MenuAction::NewRequest(folder_id) => {
            Popup::popup_name(
                state,
                String::new(),
                PopupNameAction::NewRequest(key, folder_id),
            );
            Task::none()
        }
        MenuAction::NewFolder(folder_id) => {
            Popup::popup_name(
                state,
                String::new(),
                PopupNameAction::CreateFolder(key, folder_id),
            );
            Task::none()
        }
        MenuAction::DeleteFolder(folder_id) => {
            builders::delete_folder_cmd(state, key, folder_id, move || {
                CollectionTreeMsg::ActionComplete
            })
        }
        MenuAction::RemoveCollection => {
            state.collections.remove(key);
            Task::none()
        }
        MenuAction::RenameFolder(name, folder_id) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameFolder(key, folder_id),
            );
            Task::none()
        }
        MenuAction::RenameCollection(name) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameCollection(key),
            );
            Task::none()
        }
        MenuAction::RenameRequest(name, req) => {
            Popup::popup_name(
                state,
                name.to_owned(),
                PopupNameAction::RenameRequest(key, req),
            );
            Task::none()
        }
        MenuAction::DeleteRequest(req) => {
            builders::delete_request_cmd(state, key, req, move || CollectionTreeMsg::ActionComplete)
        }
        MenuAction::CopyPath(req) => {
            if let Some(request) = state.collections.get_ref(CollectionRequest(key, req)) {
                clipboard::write(request.path.to_string_lossy().to_string())
            } else {
                Task::none()
            }
        }
        MenuAction::OpenCollection => {
            if let Some(col) = state.collections.get(key) {
                state.open_tab(Tab::Collection(CollectionTab::new(key, col)));
            }
            Task::none()
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, CollectionTreeMsg> {
    components::icon_button(ico, Some(20), Some(8)).style(button::secondary)
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

    let create_col = icon_button(icons::Plus).on_press(CollectionTreeMsg::CreateCollection);
    let open_col = icon_button(icons::FolderOpen).on_press(CollectionTreeMsg::OpenCollection);
    let settings = icon_button(icons::Gear).on_press(CollectionTreeMsg::OpenSettings);

    Column::new()
        .push(
            container(
                Row::new()
                    .push(tooltip("Create Collection", create_col))
                    .push(tooltip("Open Collection", open_col))
                    .push(tooltip("Settings", settings))
                    .width(Length::Shrink)
                    .spacing(4),
            )
            .align_x(Horizontal::Center)
            .width(Length::Fill),
        )
        .push(horizontal_line(2))
        .push(
            Scrollable::new(
                column(it)
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .spacing(4)
                    .padding(padding::right(12).bottom(12)),
            )
            .direction(Direction::Both {
                vertical: Default::default(),
                horizontal: Default::default(),
            })
            .height(Length::Fill),
        )
        .spacing(7)
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
        .padding(padding::left(12))
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
            .align_y(iced::Alignment::Center)
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
    CopyPath(RequestId),
    DeleteRequest(RequestId),
    NewRequest(Option<FolderId>),
    RenameCollection(String),
    RemoveCollection,
    OpenCollection,
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

fn context_button_request(item: &RequestRef, col: CollectionKey) -> Element<'_, CollectionTreeMsg> {
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
        .align_y(iced::Alignment::Center)
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
                "Copy Path",
                CollectionTreeMsg::ContextMenu(col, MenuAction::CopyPath(request_id)),
            ),
            menu_item(
                "Delete",
                CollectionTreeMsg::ContextMenu(col, MenuAction::DeleteRequest(request_id)),
            ),
        ],
    )
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
                "Open",
                CollectionTreeMsg::ContextMenu(col, MenuAction::OpenCollection),
            ),
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
                "Close",
                CollectionTreeMsg::ContextMenu(col, MenuAction::RemoveCollection),
            ),
        ],
    )
}
