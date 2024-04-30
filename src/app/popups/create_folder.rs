use std::borrow::Cow;

use iced::widget::{horizontal_space, text, text_input, Column, Row};
use iced::{Command, Element};

use crate::commands::builders;
use crate::state::popups::{CreateFolderState, Popup};
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    CreateFolder(String),
    Done,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(Popup::CreateFolder(data)) = state.popup.as_mut() else {
            return Command::none();
        };

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Command::none()
            }
            Message::CreateFolder(name) => {
                let CreateFolderState { col, folder_id, .. } = *data;
                builders::create_folder(state, col, folder_id, name, || Message::Done)
            }
            Message::Done => {
                state.popup = None;
                Command::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Folder Name")
}

pub fn done(data: &CreateFolderState) -> Option<Message> {
    if data.name.is_empty() {
        None
    } else {
        Some(Message::CreateFolder(data.name.clone()))
    }
}

pub(crate) fn view<'a>(_state: &'a AppState, data: &'a CreateFolderState) -> Element<'a, Message> {
    let name = Row::new()
        .push(text("Name"))
        .push(horizontal_space())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

    Column::new().push(name).spacing(4).width(300).into()
}
