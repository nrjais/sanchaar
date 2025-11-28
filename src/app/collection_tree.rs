use iced::advanced::widget;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::button::Status;
use iced::widget::space::horizontal;
use iced::widget::text::Wrapping;
use iced::widget::{Button, Column, Row, Tooltip, button, column, container, hover, row, text};
use iced::{Element, Length, Point, Rectangle, Task, clipboard, padding};

use crate::components::{
    self, NerdIcon, context_menu, horizontal_line, icon, icons, menu_item, scrollable, tooltip,
};
use crate::ids::PERF_REQUEST_DROP_ZONE;
use lib::http::collection::{Collection, Entry, FolderId, RequestId, RequestRef};
use lib::http::{CollectionKey, CollectionRequest, request::Request};

use crate::commands::builders::{self, open_collection_cmd, open_request_cmd};
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::tabs::history_tab::HistoryTab;
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
    OpenHistory,
    OpenPerformance,
    RequestDrop(Point, Rectangle, CollectionRequest),
    HandleDropZones(Vec<(widget::Id, Rectangle)>, CollectionRequest),
}

impl CollectionTreeMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let collections = &mut state.common.collections;
        match self {
            CollectionTreeMsg::ToggleExpandCollection(key) => {
                collections.with_collection_mut(key, |collection| collection.toggle_expand());
            }
            CollectionTreeMsg::ToggleFolder(col, id) => {
                collections.with_collection_mut(col, |collection| collection.toggle_folder(id));
            }
            CollectionTreeMsg::OpenRequest(col) => {
                if !state.switch_to_tab(col) {
                    return open_request_cmd(&mut state.common, col)
                        .map(move |res| Self::RequestLoaded(col, Box::new(res)));
                };
            }
            CollectionTreeMsg::CreateCollection => {
                Popup::create_collection(&mut state.common);
            }
            CollectionTreeMsg::OpenCollection => {
                return open_collection_cmd().map(Self::OpenCollectionHandle);
            }
            CollectionTreeMsg::OpenCollectionHandle(handle) => {
                if let Some(handle) = handle {
                    collections.insert(handle);
                }
            }
            CollectionTreeMsg::RequestLoaded(col, req) => {
                if let Some((req, name)) = *req {
                    state.open_tab(Tab::Http(HttpTab::new(&name, req, col)));
                }
            }
            CollectionTreeMsg::ContextMenu(col, action) => {
                return handle_context_menu(state, col, action);
            }
            CollectionTreeMsg::ActionComplete => (),
            CollectionTreeMsg::OpenHistory => {
                state.open_unique_tab(Tab::History(HistoryTab::new()));
            }
            CollectionTreeMsg::OpenPerformance => {
                state.open_tab(Tab::Perf(Box::default()));
            }
            CollectionTreeMsg::RequestDrop(point, _, request) => {
                return iced_drop::zones_on_point(
                    move |zones| CollectionTreeMsg::HandleDropZones(zones, request),
                    point,
                    Some(vec![PERF_REQUEST_DROP_ZONE]),
                    None,
                );
            }
            CollectionTreeMsg::HandleDropZones(zones, request) => {
                zones
                    .iter()
                    .find(|(zone_id, _)| *zone_id == PERF_REQUEST_DROP_ZONE)
                    .inspect(|(_, _)| {
                        if let Some(Tab::Perf(tab)) = state.active_tab_mut() {
                            tab.set_request(request);
                        }
                    });
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
    let common = &mut state.common;
    match action {
        MenuAction::NewRequest(folder_id) => {
            Popup::popup_name(
                common,
                String::new(),
                PopupNameAction::NewRequest(key, folder_id),
            );
            Task::none()
        }
        MenuAction::NewFolder(folder_id) => {
            Popup::popup_name(
                common,
                String::new(),
                PopupNameAction::CreateFolder(key, folder_id),
            );
            Task::none()
        }
        MenuAction::DeleteFolder(folder_id) => builders::delete_folder_cmd(common, key, folder_id)
            .map(move |_| CollectionTreeMsg::ActionComplete),
        MenuAction::RemoveCollection => {
            common.collections.remove(key);
            Task::none()
        }
        MenuAction::RenameFolder(name, folder_id) => {
            Popup::popup_name(
                common,
                name.to_owned(),
                PopupNameAction::RenameFolder(key, folder_id),
            );
            Task::none()
        }
        MenuAction::RenameCollection(name) => {
            Popup::popup_name(
                common,
                name.to_owned(),
                PopupNameAction::RenameCollection(key),
            );
            Task::none()
        }
        MenuAction::RenameRequest(name, req) => {
            Popup::popup_name(
                common,
                name.to_owned(),
                PopupNameAction::RenameRequest(key, req),
            );
            Task::none()
        }
        MenuAction::DeleteRequest(req) => builders::delete_request_cmd(common, key, req)
            .map(move |_| CollectionTreeMsg::ActionComplete),
        MenuAction::CopyPath(req) => {
            if let Some(request) = common.collections.get_ref(CollectionRequest(key, req)) {
                clipboard::write(request.path.to_string_lossy().to_string())
            } else {
                Task::none()
            }
        }
        MenuAction::OpenCollection => {
            if let Some(col) = common.collections.get(key) {
                let tab = Tab::Collection(CollectionTab::new(key, col));
                state.open_tab(tab);
            }
            Task::none()
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, CollectionTreeMsg> {
    components::icon_button(ico, Some(22), Some(8)).style(move |theme, status| {
        if status == Status::Hovered || status == Status::Pressed {
            button::Style {
                text_color: theme.extended_palette().primary.strong.color,
                ..button::text(theme, status)
            }
        } else {
            button::text(theme, status)
        }
    })
}

pub fn view(state: &AppState) -> Element<CollectionTreeMsg> {
    let tree = state.common.collections.iter().map(|(key, collection)| {
        expandable(
            key,
            &collection.name,
            &collection.entries,
            collection.expanded,
            CollectionTreeMsg::ToggleExpandCollection(key),
            None,
            0,
        )
    });

    let tree = scrollable(
        column(tree)
            .spacing(4)
            .padding(padding::bottom(4).left(4).top(4)),
    );

    let create_col = icon_button(icons::Plus).on_press(CollectionTreeMsg::CreateCollection);
    let open_col = icon_button(icons::FolderOpen).on_press(CollectionTreeMsg::OpenCollection);
    let history = icon_button(icons::History).on_press(CollectionTreeMsg::OpenHistory);
    let perf = icon_button(icons::Speedometer).on_press(CollectionTreeMsg::OpenPerformance);

    Column::new()
        .push(
            container(
                Row::new()
                    .push(tooltip("Create Collection", create_col))
                    .push(tooltip("Open Collection", open_col))
                    .push(tooltip("History", history))
                    .push(tooltip("Performance", perf))
                    .width(Length::Shrink)
                    .align_y(Vertical::Center)
                    .spacing(8),
            )
            .align_x(Horizontal::Center)
            .padding(padding::top(4).bottom(4))
            .width(Length::Fill),
        )
        .push(horizontal_line(2))
        .push(tree)
        .into()
}

fn folder_tree(col: CollectionKey, entries: &[Entry], indent: u32) -> Element<CollectionTreeMsg> {
    let it = entries.iter().map(|entry| match entry {
        Entry::Item(item) => context_button_request(item, col, indent),
        Entry::Folder(folder) => expandable(
            col,
            &folder.name,
            &folder.entries,
            folder.expanded,
            CollectionTreeMsg::ToggleFolder(col, folder.id),
            Some(folder.id),
            indent,
        ),
    });

    column(it).spacing(2).width(Length::Fill).into()
}

#[allow(clippy::too_many_arguments)]
fn expandable<'a>(
    col: CollectionKey,
    name: &'a str,
    entries: &'a [Entry],
    expanded: bool,
    on_expand_toggle: CollectionTreeMsg,
    folder_id: Option<FolderId>,
    indent: u32,
) -> Element<'a, CollectionTreeMsg> {
    if expanded {
        let children = folder_tree(col, entries, indent + 1);
        Column::new()
            .push(expandable_button(
                name,
                on_expand_toggle,
                icons::FolderOpen,
                col,
                folder_id,
                indent,
            ))
            .push(children)
            .spacing(2)
            .into()
    } else {
        expandable_button(
            name,
            on_expand_toggle,
            icons::Folder,
            col,
            folder_id,
            indent,
        )
        .into()
    }
}

fn action_button<'a>(
    desc: &'a str,
    ico: NerdIcon,
    msg: CollectionTreeMsg,
) -> Tooltip<'a, CollectionTreeMsg> {
    tooltip(
        desc,
        components::icon_button(ico, Some(16), Some(4))
            .on_press(msg)
            .width(Length::Shrink)
            .style(move |theme, status| {
                if status == Status::Hovered || status == Status::Pressed {
                    button::Style {
                        text_color: theme.extended_palette().primary.strong.color,
                        ..button::text(theme, status)
                    }
                } else {
                    button::text(theme, status)
                }
            }),
    )
}

fn expandable_button(
    name: &str,
    on_expand_toggle: CollectionTreeMsg,
    arrow: NerdIcon,
    col: CollectionKey,
    folder_id: Option<FolderId>,
    indent: u32,
) -> impl Into<Element<CollectionTreeMsg>> {
    let base = button(
        row([
            icon(arrow)
                .size(18)
                .wrapping(Wrapping::None)
                .align_x(iced::Alignment::Start)
                .into(),
            text(name).wrapping(Wrapping::None).size(16).into(),
        ])
        .align_y(iced::Alignment::Center)
        .clip(true)
        .spacing(8),
    )
    .style(|theme, status| {
        if status == Status::Hovered || status == Status::Pressed {
            button::subtle(theme, Status::Hovered)
        } else {
            button::text(theme, status)
        }
    })
    .on_press(on_expand_toggle)
    .width(Length::Fill)
    .padding(padding::left(12 * indent + 4));

    let base = if let Some(folder_id) = folder_id {
        context_button_folder(base, name.to_owned(), col, folder_id)
    } else {
        context_button_collection(base, name.to_owned(), col)
    };

    let actions = Row::new()
        .push(horizontal())
        .push(action_button(
            "New Request",
            icons::Plus,
            CollectionTreeMsg::ContextMenu(col, MenuAction::NewRequest(folder_id)),
        ))
        .align_y(Vertical::Center)
        .padding(padding::right(4))
        .height(Length::Fill);

    hover(base, actions)
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
    .into()
}

fn context_button_request(
    item: &RequestRef,
    col: CollectionKey,
    indent: u32,
) -> Element<'_, CollectionTreeMsg> {
    let collection_request = CollectionRequest(col, item.id);

    let base = row([
        icon(icons::API)
            .size(16)
            .style(|t| text::Style {
                color: Some(t.extended_palette().success.strong.color),
            })
            .align_x(iced::Alignment::Start)
            .into(),
        text(&item.name).wrapping(Wrapping::None).size(16).into(),
    ])
    .align_y(iced::Alignment::Center)
    .width(Length::Fill)
    .clip(true)
    .spacing(8);

    let droppable = iced_drop::droppable(base)
        .on_press(CollectionTreeMsg::OpenRequest(collection_request))
        .on_drop(move |point, bounds| {
            CollectionTreeMsg::RequestDrop(point, bounds, collection_request)
        });

    let base = button(droppable)
        .on_press(CollectionTreeMsg::OpenRequest(collection_request))
        .style(|theme, status| {
            if status == Status::Hovered || status == Status::Pressed {
                button::subtle(theme, Status::Hovered)
            } else {
                button::text(theme, Status::Active)
            }
        })
        .padding(padding::left(12 * indent + 4))
        .width(Length::Fill);

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
    .into()
}
