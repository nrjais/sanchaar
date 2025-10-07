use iced::widget::{Column, text};
use iced::{Element, Task};
use iced_auto_updater_plugin::AutoUpdaterMessage;
use std::borrow::Cow;

use crate::app::AppMsg;
use crate::state::AppState;
use crate::state::popups::{Popup, UpdateConfirmationState};

use super::PopupMsg;

#[derive(Debug, Clone)]
pub enum Message {
    Confirm,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<PopupMsg> {
        match self {
            Self::Confirm => {
                Popup::close(&mut state.common);

                if let Some(release) = state.pending_release.clone() {
                    let msg = state
                        .plugins
                        .auto_updater
                        .message(AutoUpdaterMessage::DownloadAndInstall(release));
                    state.queue.push(AppMsg::Plugin(msg));
                }
                Task::none()
            }
        }
    }
}

pub fn title() -> Cow<'static, str> {
    Cow::Borrowed("Update Available")
}

pub fn view<'a>(
    state: &'a AppState,
    _popup_state: &'a UpdateConfirmationState,
) -> Element<'a, Message> {
    let (message, details) = if let Some(ref release) = state.pending_release {
        let version = &release.tag_name;
        (
            format!("A new version {} is available!", version),
            "Would you like to download and install it?",
        )
    } else {
        (
            "A new version is available!".to_string(),
            "Would you like to download and install it?",
        )
    };

    Column::new()
        .push(text(message).size(16))
        .push(text(details).size(14))
        .spacing(8)
        .into()
}

pub fn done(_popup_state: &UpdateConfirmationState) -> Option<Message> {
    Some(Message::Confirm)
}
