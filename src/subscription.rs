use iced::Subscription;

use crate::{app::AppMsg, hotkeys, state::AppState, window};

pub fn all(state: &AppState) -> Subscription<AppMsg> {
    Subscription::batch([hotkeys::subscription(state), window::subscription(state)])
}
