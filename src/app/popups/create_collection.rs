use core::http::Collection;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{Column, Row, button, space, text, text_input};
use iced::{Element, Task};
use rfd::FileHandle;

use crate::commands::builders;
use crate::commands::dialog::{open_file_dialog_with_filter, open_folder_dialog};
use crate::components::{button_tab, button_tabs};
use crate::state::AppState;
use crate::state::popups::Popup::CreateCollection;
use crate::state::popups::{CollectionCreationMode, CreateCollectionState};

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    NameChanged(String),
    OpenFolderDialog,
    OpenFileDialog,
    FolderSelected(Option<Arc<FileHandle>>),
    FileSelected(Option<Arc<FileHandle>>),
    CreateCollection(String, PathBuf),
    ImportCollection(PathBuf, String),
    ModeChanged(CollectionCreationMode),
    OpenCollection(Option<Collection>),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let Some(CreateCollection(data)) = state.common.popup.as_mut() else {
            return Task::none();
        };

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Task::none()
            }
            Message::ModeChanged(mode) => {
                data.mode = mode;
                Task::none()
            }
            Message::OpenFolderDialog => {
                open_folder_dialog("Select location").map(Message::FolderSelected)
            }
            Message::OpenFileDialog => {
                open_file_dialog_with_filter("Select Postman Collection", &["json"])
                    .map(Message::FileSelected)
            }
            Message::FolderSelected(handle) => {
                data.path = handle.map(|h| h.path().to_owned());
                Task::none()
            }
            Message::OpenCollection(collection) => {
                if let Some(collection) = collection {
                    state.common.collections.insert(collection);
                }
                state.common.popup = None;
                Task::none()
            }
            Message::FileSelected(handle) => {
                if let Some(h) = handle {
                    data.import_file_path = Some(h.path().to_owned());
                    if data.name.is_empty() {
                        let file_name = h.file_name();
                        data.name = file_name.replace(".json", "");
                    }
                }
                Task::none()
            }
            Message::CreateCollection(name, path) => {
                builders::create_collection_cmd(&mut state.common, name, path)
                    .map(|_| Message::Done)
            }
            Message::ImportCollection(file_path, collection_name) => {
                let collection_path = PathBuf::from(format!("collections/{}", collection_name));
                builders::import_postman_collection_cmd(
                    &mut state.common,
                    file_path,
                    collection_path,
                )
                .map(Message::OpenCollection)
            }
            Message::Done => {
                state.common.popup = None;
                Task::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Create Collection")
}

pub fn done(data: &CreateCollectionState) -> Option<Message> {
    if data.name.trim().is_empty() {
        return None;
    }

    match data.mode {
        CollectionCreationMode::New => data
            .path
            .as_ref()
            .map(|path| Message::CreateCollection(data.name.clone(), path.clone())),
        CollectionCreationMode::ImportPostman => data
            .import_file_path
            .as_ref()
            .map(|path| Message::ImportCollection(path.clone(), data.name.clone())),
    }
}

pub(crate) fn view<'a>(
    _state: &'a AppState,
    data: &'a CreateCollectionState,
) -> Element<'a, Message> {
    let mode_tabs = button_tabs(
        data.mode,
        [
            button_tab(CollectionCreationMode::New, || text("New Collection")),
            button_tab(CollectionCreationMode::ImportPostman, || {
                text("Import from Postman")
            }),
        ]
        .into_iter(),
        Message::ModeChanged,
        None,
    );

    let name = Row::new()
        .push(text("Name"))
        .push(space::horizontal())
        .push(
            text_input("Collection name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

    let content = match data.mode {
        CollectionCreationMode::New => {
            let path = Row::new()
                .push(text("Location"))
                .push(space::horizontal())
                .push(
                    button(
                        text(
                            data.path
                                .as_ref()
                                .and_then(|p| p.to_str())
                                .unwrap_or("Browse location"),
                        )
                        .size(16),
                    )
                    .style(button::subtle)
                    .padding([2, 6])
                    .on_press(Message::OpenFolderDialog),
                )
                .align_y(iced::Alignment::Center)
                .spacing(4);

            Column::new().push(name).push(path).spacing(4)
        }
        CollectionCreationMode::ImportPostman => {
            let file_path = Row::new()
                .push(text("File"))
                .push(space::horizontal())
                .push(
                    button(
                        text(
                            data.import_file_path
                                .as_ref()
                                .and_then(|p| p.to_str())
                                .unwrap_or("Browse for Postman collection"),
                        )
                        .size(16),
                    )
                    .style(button::subtle)
                    .padding([2, 6])
                    .on_press(Message::OpenFileDialog),
                )
                .align_y(iced::Alignment::Center)
                .spacing(4);

            Column::new().push(file_path).push(name).spacing(4)
        }
    };

    Column::new()
        .push(mode_tabs)
        .push(content)
        .spacing(8)
        .width(500)
        .into()
}
