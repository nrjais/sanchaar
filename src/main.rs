pub mod app;
pub mod commands;
pub mod hotkeys;
pub mod state;

const HACK_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Regular.ttf");
const HACK_MONO_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFontMono-Regular.ttf");

use std::borrow::Cow;

use app::AppMsg;
use iced::{
    window::{self, Position},
    Point, Size, Task,
};
use state::AppState;

fn main() {
    dioxus_devtools::connect_subsecond();
    match app() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
}

fn update_hot(state: &mut AppState, msg: AppMsg) -> Task<AppMsg> {
    subsecond::HotFn::current(|(state, msg): (&mut AppState, AppMsg)| app::update(state, msg))
        .call(((state, msg),))
}

fn view_hot(state: &AppState) -> iced::Element<AppMsg> {
    subsecond::HotFn::current(|state| app::view(state)).call((state,))
}

pub fn app() -> Result<(), iced::Error> {
    iced::application(
        || (AppState::new(), commands::init_command()),
        update_hot,
        view_hot,
    )
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
