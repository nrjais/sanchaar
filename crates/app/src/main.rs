use std::borrow::Cow;

use iced::{
    window::{self, Position},
    Point, Size, Theme,
};

use commands::init_command;
use state::AppState;

pub mod app;
pub mod commands;
pub mod state;

pub const HACK_REG_BYTES: &[u8] = include_bytes!("../../../fonts/HackNerdFont-Regular.ttf");

fn main() -> iced::Result {
    iced::application("Sanchaar", app::update, app::view)
        .theme(theme)
        .load(init_command)
        .antialiasing(true)
        .font(Cow::from(HACK_REG_BYTES))
        .window(window::Settings {
            size: Size::new(1024.0, 768.0),
            position: Position::Specific(Point::ORIGIN),
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        })
        .run()
}

fn theme(state: &AppState) -> Theme {
    state.theme.clone()
}
