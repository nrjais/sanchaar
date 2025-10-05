#![allow(mismatched_lifetime_syntaxes)]
pub mod app;
pub mod commands;
pub mod components;
pub mod hotkeys;
pub mod state;
mod subscription;

use iced::{
    Size,
    window::{Position, Settings},
};
use iced_window_state_plugin::WindowStatePlugin;
use state::AppState;
use std::borrow::Cow;

const HACK_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Regular.ttf");
const APP_NAME: &str = "Sanchaar";

fn main() {
    env_logger::init();
    match app() {
        Ok(_) => (),
        Err(e) => {
            log::error!("{e}");
            std::process::exit(1);
        }
    };
}

pub fn app() -> Result<(), iced::Error> {
    let window_state = WindowStatePlugin::load(APP_NAME);
    let maximized = window_state.is_none();
    let window_state = window_state.unwrap_or_default();

    let state_init = { move || (AppState::new(), commands::init_command()) };

    iced::application(state_init, app::update, app::view)
        .theme(AppState::theme)
        .antialiasing(true)
        .subscription(subscription::all)
        .font(Cow::from(HACK_REG_BYTES))
        .window(Settings {
            size: window_state.size,
            position: Position::Specific(window_state.position),
            maximized,
            min_size: Some(Size::new(900.0, 600.0)),
            ..Default::default()
        })
        .title("Sanchaar")
        .run()
}
