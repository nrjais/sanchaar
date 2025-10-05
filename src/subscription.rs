use iced::Subscription;

use crate::{app::AppMsg, hotkeys, state::AppState};

pub fn all(state: &AppState) -> Subscription<AppMsg> {
    let plugin_subscription = state.manager.subscriptions().map(AppMsg::Plugin);
    Subscription::batch([hotkeys::subscription(state), plugin_subscription])
}
