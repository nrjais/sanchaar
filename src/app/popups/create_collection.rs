use std::borrow::Cow;

use iced::widget::{horizontal_space, text, text_input, Column, Row};
use iced::{Element, Task};

use crate::commands::builders;
use crate::state::popups::CreateCollectionState;
use crate::state::popups::Popup::CreateCollection;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    NameChanged(String),
    CreateCollection(String),
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
            Message::CreateCollection(name) => {
                builders::create_collection_cmd(&mut state.common, name).map(|_| Message::Done)
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
        Some(Message::CreateCollection(data.name.clone()))
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

    Column::new().push(name).spacing(4).width(300).into()
}
