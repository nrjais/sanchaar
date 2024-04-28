use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{button, horizontal_space, text, text_input, Column, Row};
use iced::{Command, Element};
use rfd::FileHandle;

use crate::commands::builders;
use crate::commands::dialog::open_folder_dialog;
use crate::state::popups::CreateCollectionState;
use crate::state::popups::Popup::CreateCollection;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    NameChanged(String),
    OpenDialog,
    FolderSelected(Option<Arc<FileHandle>>),
    CreateCollection(String, PathBuf),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(CreateCollection(data)) = state.popup.as_mut() else {
            return Command::none();
        };

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Command::none()
            }
            Message::OpenDialog => open_folder_dialog("Select location", Message::FolderSelected),
            Message::FolderSelected(handle) => {
                data.path = handle.map(|h| h.path().to_owned());
                Command::none()
            }
            Message::CreateCollection(name, path) => {
                builders::create_collection(state, name, path, |_| Message::Done)
            }
            Message::Done => {
                state.popup = None;
                Command::none()
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
        .push(horizontal_space())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

    let path = Row::new()
        .push(text("Location"))
        .push(horizontal_space())
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
            .style(button::text)
            .padding([2, 6])
            .on_press(Message::OpenDialog),
        )
        .align_items(iced::Alignment::Center)
        .spacing(4);

    Column::new()
        .push(name)
        .push(path)
        .spacing(4)
        .width(300)
        .into()
}
