use std::borrow::Cow;

use iced::{
    window::{self, Position},
    Element, Point, Size, Task, Theme,
};

use app::AppMsg;
use commands::init_command;
use state::AppState;

pub mod app;
pub mod commands;
pub mod state;

pub const HACK_REG_BYTES: &[u8] = include_bytes!("../../../fonts/HackNerdFont-Regular.ttf");

fn main() -> iced::Result {
    iced::application("Sanchaar", update, view)
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

fn update(state: &mut AppState, message: AppMsg) -> Task<AppMsg> {
    message.update(state)
}

fn view(state: &AppState) -> Element<AppMsg> {
    app::view(state)
}

fn theme(state: &AppState) -> Theme {
    state.theme.clone()
}
