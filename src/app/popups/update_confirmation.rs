use iced::widget::{Column, text};
use iced::{Element, Task};
use iced_auto_updater_plugin::{AutoUpdaterMessage, ReleaseInfo};
use std::borrow::Cow;

use crate::app::AppMsg;
use crate::state::AppState;
use crate::state::popups::{Popup, UpdateConfirmationState};

use super::PopupMsg;

#[derive(Debug, Clone)]
pub enum Message {
    Confirm(ReleaseInfo),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<PopupMsg> {
        match self {
            Self::Confirm(release) => {
                Popup::close(&mut state.common);

                let msg = state
                    .plugins
                    .auto_updater
                    .message(AutoUpdaterMessage::DownloadAndInstall(release.clone()));
                state.queue.push(AppMsg::Plugin(msg));

                Task::none()
            }
        }
    }
}

pub fn title() -> Cow<'static, str> {
    Cow::Borrowed("Update Available")
}

pub fn view<'a>(popup_state: &'a UpdateConfirmationState) -> Element<'a, Message> {
    let version = &popup_state.0.tag_name;
    Column::new()
        .push(text("New update available to install!".to_string()).size(16))
        .push(text(format!("Updated version: {}, ", version)).size(12))
        .push(text(format!("Current version: {}", env!("CARGO_PKG_VERSION"))).size(12))
        .spacing(8)
        .width(400)
        .into()
}

pub fn done(popup_state: &UpdateConfirmationState) -> Option<Message> {
    Some(Message::Confirm(popup_state.0.clone()))
}
