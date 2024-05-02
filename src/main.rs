use std::borrow::Cow;

use iced::advanced::Application;
use iced::{
    window::{self, Position},
    Command, Element, Point, Renderer, Settings, Size, Theme,
};

use app::AppMsg;
use commands::init_command;
use state::AppState;

pub mod app;
pub mod commands;
pub mod state;

pub const HACK_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Regular.ttf");
pub const HACK_BOLD_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Bold.ttf");

fn main() -> iced::Result {
    Sanchaar::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: Size::new(1024.0, 768.0),
            position: Position::Specific(Point::ORIGIN),
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        },
        fonts: Vec::from([Cow::from(HACK_REG_BYTES), Cow::from(HACK_BOLD_BYTES)]),
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
pub struct Sanchaar {
    state: AppState,
}

impl Application for Sanchaar {
    type Executor = iced::executor::Default;
    type Message = AppMsg;
    type Theme = Theme;
    type Renderer = Renderer;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Sanchaar, Command<AppMsg>) {
        (Sanchaar::default(), init_command())
    }

    fn title(&self) -> String {
        String::from("Sanchaar")
    }

    fn update(&mut self, message: AppMsg) -> Command<AppMsg> {
        message.update(&mut self.state)
    }

    fn view(&self) -> Element<AppMsg> {
        app::view(&self.state)
    }

    fn theme(&self) -> Self::Theme {
        self.state.theme.clone()
    }
}
