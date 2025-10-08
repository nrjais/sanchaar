use iced::Subscription;

use crate::{app::AppMsg, hotkeys, state::AppState};

pub fn all(state: &AppState) -> Subscription<AppMsg> {
    Subscription::batch([
        state.plugins.manager.subscriptions().map(AppMsg::Plugin),
        state.plugins.auto_updater.listen().map(AppMsg::AutoUpdater),
        hotkeys::subscription(state),
    ])
}
