#![allow(mismatched_lifetime_syntaxes)]
pub mod app;
pub mod commands;
pub mod components;
pub mod hotkeys;
pub mod state;
mod subscription;
mod window;

const HACK_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Regular.ttf");

use std::borrow::Cow;

use iced::{
    Point, Size,
    window::{Position, Settings},
};
use state::AppState;

use crate::window::load_window_state;

fn main() {
    match app() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
}

pub fn app() -> Result<(), iced::Error> {
    env_logger::init();
    let (window_state, maximized) = load_window_state();

    let state_init = {
        let window_state = window_state.clone();
        move || {
            (
                AppState::new(window_state.clone()),
                commands::init_command(),
            )
        }
    };

    iced::application(state_init, app::update, app::view)
        .theme(AppState::theme)
        .antialiasing(true)
        .subscription(subscription::all)
        .font(Cow::from(HACK_REG_BYTES))
        .window(Settings {
            size: Size::new(window_state.width, window_state.height),
            position: Position::Specific(Point::new(window_state.x, window_state.y)),
            maximized,
            min_size: Some(Size::new(900.0, 600.0)),
            ..Default::default()
        })
        .title("Sanchaar")
        .run()
}
