use core::http::CollectionRequest;
use std::borrow::Cow;

use iced::widget::{horizontal_space, text, text_input, Column, Row};
use iced::{Command, Element};

use crate::commands::builders;
use crate::state::popups::{Popup, PopupNameAction, PopupNameState};
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Rename(String),
    Done,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(Popup::PopupName(data)) = state.popup.as_mut() else {
            return Command::none();
        };

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Command::none()
            }
            Message::Rename(name) => match data.action {
                PopupNameAction::RenameCollection(col) => {
                    builders::rename_collection_cmd(state, col, name, || Message::Done)
                }
                PopupNameAction::RenameFolder(col, folder_id) => {
                    builders::rename_folder_cmd(state, col, folder_id, name, || Message::Done)
                }
                PopupNameAction::CreateFolder(col, folder_id) => {
                    builders::create_folder_cmd(state, col, folder_id, name, || Message::Done)
                }
                PopupNameAction::RenameRequest(col, req) => {
                    builders::rename_request_cmd(state, CollectionRequest(col, req), name, || {
                        Message::Done
                    })
                }
            },
            Message::Done => {
                state.popup = None;
                Command::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Enter Name")
}

pub fn done(data: &PopupNameState) -> Option<Message> {
    if data.name.is_empty() {
        None
    } else {
        Some(Message::Rename(data.name.clone()))
    }
}

pub(crate) fn view<'a>(_state: &'a AppState, data: &'a PopupNameState) -> Element<'a, Message> {
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
