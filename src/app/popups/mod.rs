use crate::state::{AppState, Popup};
use iced::widget::{button, container, horizontal_space, text, Column, Row};
use iced::Element;

mod create_collection;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
    ClosePopup,
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::CreateCollection(msg) => msg.update(state),
            Self::ClosePopup => state.popup = None,
        }
    }
}

pub fn view(state: &AppState, popup: Popup) -> Element<PopupMsg> {
    let (title, content, done_msg) = match popup {
        Popup::CreateCollection => (
            create_collection::title(),
            create_collection::view(state).map(PopupMsg::CreateCollection),
            PopupMsg::CreateCollection(create_collection::Message::Done),
        ),
    };

    let buttons = Row::new()
        .push(horizontal_space())
        .push(
            button("Cancel")
                .style(button::secondary)
                .on_press(PopupMsg::ClosePopup),
        )
        .push(button("Done").style(button::primary).on_press(done_msg))
        .spacing(8);

    container(
        Column::new()
            .push(
                Column::new()
                    .push(text(title).size(24))
                    .padding([0, 4])
                    .align_items(iced::Alignment::Center),
            )
            .push(content)
            .push(Column::new().push(buttons).spacing(4))
            .width(iced::Length::Shrink)
            .spacing(8),
    )
    .padding(8)
    .style(container::rounded_box)
    .into()
}
