use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{Column, Row, button, space, text, text_input};
use iced::{Element, Task};
use rfd::FileHandle;

use crate::commands::builders;
use crate::commands::dialog::open_folder_dialog;
use crate::state::AppState;
use crate::state::popups::CreateCollectionState;
use crate::state::popups::Popup::CreateCollection;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    NameChanged(String),
    OpenDialog,
    FolderSelected(Option<Arc<FileHandle>>),
    CreateCollection(String, PathBuf),
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
            Message::OpenDialog => {
                open_folder_dialog("Select location").map(Message::FolderSelected)
            }
            Message::FolderSelected(handle) => {
                data.path = handle.map(|h| h.path().to_owned());
                Task::none()
            }
            Message::CreateCollection(name, path) => {
                builders::create_collection_cmd(&mut state.common, name, path)
                    .map(|_| Message::Done)
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
    if data.name.is_empty() {
        None
    } else {
        data.path
            .as_ref()
            .map(|path| Message::CreateCollection(data.name.clone(), path.clone()))
    }
}

pub(crate) fn view<'a>(
    _state: &'a AppState,
    data: &'a CreateCollectionState,
) -> Element<'a, Message> {
    let name = Row::new()
        .push(text("Name"))
        .push(space::horizontal())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

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
            .on_press(Message::OpenDialog),
        )
        .align_y(iced::Alignment::Center)
        .spacing(4);

    Column::new()
        .push(name)
        .push(path)
        .spacing(4)
        .width(400)
        .into()
}
