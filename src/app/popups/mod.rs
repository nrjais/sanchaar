use iced::alignment::Horizontal;
use iced::widget::container::Style;
use iced::widget::{button, container, horizontal_space, text, Column, Row};
use iced::{Border, Command, Element};

use crate::state::popups::Popup;
use crate::state::AppState;

mod create_collection;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
    ClosePopup,
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) -> Command<PopupMsg> {
        match self {
            Self::CreateCollection(msg) => msg.update(state).map(PopupMsg::CreateCollection),
            Self::ClosePopup => {
                state.popup = None;
                Command::none()
            }
        }
    }
}

pub fn view<'a>(state: &'a AppState, popup: &'a Popup) -> Element<'a, PopupMsg> {
    let (title, content, done_msg) = match popup {
        Popup::CreateCollection(ref data) => (
            create_collection::title(),
            create_collection::view(state, data).map(PopupMsg::CreateCollection),
            create_collection::done(data).map(PopupMsg::CreateCollection),
        ),
    };

    let buttons = Row::new()
        .push(horizontal_space())
        .push(
            button("Cancel")
                .style(button::secondary)
                .on_press(PopupMsg::ClosePopup),
        )
        .push(
            button("Done")
                .style(button::primary)
                .on_press_maybe(done_msg),
        )
        .spacing(8);

    Row::new()
        .push(horizontal_space())
        .push(
            container(
                Column::new()
                    .push(
                        container(text(title).size(20))
                            .width(iced::Length::Fill)
                            .align_x(Horizontal::Center),
                    )
                    .push(content)
                    .push(buttons)
                    .spacing(12),
            )
            .padding(16)
            .style(|theme| Style {
                background: Some(theme.extended_palette().background.weak.color.into()),
                border: Border::rounded(6),
                ..Style::default()
            }),
        )
        .push(horizontal_space())
        .align_items(iced::Alignment::Center)
        .into()
}
