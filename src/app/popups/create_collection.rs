use std::borrow::Cow;
use std::sync::Arc;

use iced::widget::{button, horizontal_space, text, text_input, Column, Row};
use iced::{Command, Element};
use rfd::FileHandle;

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
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(popup) = state.popup.as_mut() else {
            return Command::none();
        };
        let CreateCollection(data) = popup;

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Command::none()
            }
            Message::OpenDialog => open_folder_dialog("Select location", Message::FolderSelected),
            Message::FolderSelected(Some(handle)) => {
                data.path = Some(handle.path().to_owned());
                Command::none()
            }
            _ => Command::none(),
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Create Collection")
}

pub fn done(data: &CreateCollectionState) -> Option<Message> {
    if data.name.is_empty() || data.path.is_none() {
        None
    } else {
        Some(Message::Done)
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
                data.path
                    .as_ref()
                    .and_then(|p| p.to_str())
                    .unwrap_or("Browse location"),
            )
            .style(button::text)
            .padding([2, 6])
            .on_press(Message::OpenDialog),
        )
        .align_items(iced::Alignment::Center)
        .spacing(4);

    Column::new().push(name).push(path).spacing(4).into()
}
