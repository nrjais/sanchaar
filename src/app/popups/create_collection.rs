use std::borrow::Cow;

use iced::widget::{button, horizontal_space, text, text_input, Column, Row};
use iced::Element;

use crate::state::popups::CreateCollectionState;
use crate::state::popups::Popup::CreateCollection;
use crate::state::AppState;

#[derive(Clone, Debug)]
pub enum Message {
    Done,
    NameChanged(String),
    OpenDialog,
}

impl Message {
    pub fn update(self, state: &mut AppState) {
        let Some(popup) = state.popup.as_mut() else {
            return;
        };
        let CreateCollection(data) = popup;

        match self {
            Message::Done => {}
            Message::NameChanged(name) => {
                data.name = name;
            }
            Message::OpenDialog => {}
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Create Collection")
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
            button("Browse location")
                .style(button::text)
                .padding([2, 6])
                .on_press(Message::OpenDialog),
        )
        .align_items(iced::Alignment::Center)
        .spacing(4);

    Column::new().push(name).push(path).spacing(4).into()
}
