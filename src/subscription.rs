use iced::Subscription;

use crate::{app::AppMsg, hotkeys, state::AppState};

pub fn all(state: &AppState) -> Subscription<AppMsg> {
    let plugin_subscription = state.plugins.auto_updater.listen().map(AppMsg::AutoUpdater);
    Subscription::batch([hotkeys::subscription(state), plugin_subscription])
}
