use iced::widget::container::Style;
use iced::widget::{button, container, horizontal_space, text, Column, Row};
use iced::Length::{Fill, Shrink};
use iced::{border, Alignment, Element, Task};

use crate::state::popups::Popup;
use crate::state::AppState;

mod app_settings;
mod create_collection;
mod name_popup;
mod save_request;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
    SaveRequest(save_request::Message),
    RenamePopup(name_popup::Message),
    AppSettings(app_settings::Message),
    ClosePopup,
    Ignore,
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) -> Task<PopupMsg> {
        match self {
            PopupMsg::CreateCollection(msg) => msg.update(state).map(PopupMsg::CreateCollection),
            PopupMsg::SaveRequest(msg) => msg.update(state).map(PopupMsg::SaveRequest),
            PopupMsg::RenamePopup(msg) => msg.update(state).map(PopupMsg::RenamePopup),
            PopupMsg::AppSettings(msg) => msg.update(state).map(PopupMsg::AppSettings),
            PopupMsg::ClosePopup => {
                Popup::close(&mut state.common);
                Task::none()
            }
            PopupMsg::Ignore => Task::none(),
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
        Popup::SaveRequest(data) => (
            save_request::title(),
            save_request::view(state, data).map(PopupMsg::SaveRequest),
            save_request::done(data).map(PopupMsg::SaveRequest),
        ),
        Popup::PopupName(data) => (
            name_popup::title(),
            name_popup::view(state, data).map(PopupMsg::RenamePopup),
            name_popup::done(data).map(PopupMsg::RenamePopup),
        ),
        Popup::AppSettings(data) => (
            app_settings::title(),
            app_settings::view(state, data).map(PopupMsg::AppSettings),
            app_settings::done(data).map(PopupMsg::AppSettings),
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
        .width(Fill)
        .height(Shrink)
        .align_y(Alignment::End)
        .spacing(8);

    container(
        Column::new()
            .push(text(title).size(20))
            .push(content)
            .push(buttons)
            .width(Shrink)
            .height(Shrink)
            .spacing(12),
    )
    .padding(16)
    .style(|theme| Style {
        background: Some(theme.extended_palette().background.weak.color.into()),
        border: border::rounded(6),
        ..Style::default()
    })
    .into()
}
