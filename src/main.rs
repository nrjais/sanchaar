pub mod app;
pub mod commands;
pub mod components;
pub mod core;
pub mod panels;
pub mod state;
pub mod transformers;

use app::AppMsg;
use iced::{
    window::{self, Position},
    Application, Command, Element, Point, Settings, Size, Theme,
};
use state::AppState;

fn main() -> iced::Result {
    Sanchaar::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: Size::new(1024.0, 768.0),
            position: Position::Specific(Point::ORIGIN),
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
pub struct Sanchaar {
    state: AppState,
}

impl Application for Sanchaar {
    type Executor = iced::executor::Default;
    type Message = AppMsg;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Sanchaar, Command<AppMsg>) {
        (
            Sanchaar {
                state: AppState::new(),
            },
            Command::none(),
        )
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
        Theme::Dark
    }
}
