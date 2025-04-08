use std::borrow::Cow;

use iced::{
    window::{self, Position},
    Point, Size,
};

use commands::init_command;
use state::AppState;

mod app;
mod commands;
mod hotkeys;
mod state;

const HACK_REG_BYTES: &[u8] = include_bytes!("../../../fonts/HackNerdFont-Regular.ttf");
const HACK_MONO_REG_BYTES: &[u8] = include_bytes!("../../../fonts/HackNerdFontMono-Regular.ttf");

pub fn main() -> Result<(), iced::Error> {
    iced::application(|| (AppState::new(), init_command()), app::update, app::view)
        .theme(|s| s.theme.clone())
        .antialiasing(true)
        .subscription(hotkeys::subscription)
        .font(Cow::from(HACK_REG_BYTES))
        .font(Cow::from(HACK_MONO_REG_BYTES))
        .window(window::Settings {
            size: Size::new(1024.0, 768.0),
            position: Position::Specific(Point::ORIGIN),
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        })
        .title("Sanchaar")
        .run()
}
