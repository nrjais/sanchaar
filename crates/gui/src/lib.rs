use std::borrow::Cow;

use iced::{
    window::{self, Position},
    Point, Size,
};

use commands::init_command;
use state::AppState;

pub(crate) mod app;
pub(crate) mod commands;
pub(crate) mod state;

const HACK_REG_BYTES: &[u8] = include_bytes!("../../../fonts/HackNerdFont-Regular.ttf");

pub fn main() -> Result<(), iced::Error> {
    iced::application("Sanchaar", app::update, app::view)
        .theme(|s| s.theme.clone())
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
