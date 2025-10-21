#![allow(mismatched_lifetime_syntaxes)]

pub mod app;
pub mod commands;
pub mod components;
mod debug;
pub mod hotkeys;
pub mod ids;
pub mod state;
pub mod subscription;
pub mod widgets;

use iced::{
    Size, Task,
    window::{Position, Settings},
};
use iced_window_state_plugin::{AppName, WindowState, WindowStatePlugin};
use lib::APP_NAME;
use state::AppState;
use std::borrow::Cow;
use tokio::runtime::Runtime;

use crate::{app::AppMsg, state::install_plugins};

const HACK_REG_BYTES: &[u8] = include_bytes!("../fonts/HackNerdFont-Regular.ttf");

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

fn load_window_state() -> Option<WindowState> {
    let app_name = AppName::new("com", "nrjais", APP_NAME);
    let rt = Runtime::new().unwrap();

    rt.block_on(WindowStatePlugin::load(&app_name))
}

pub fn app() -> Result<(), iced::Error> {
    let window_state = load_window_state();
    let maximized = window_state.is_none();
    let window_state = window_state.unwrap_or_default();

    let state_init = {
        move || {
            let (plugins, task) = install_plugins();
            (
                AppState::new(plugins),
                Task::batch([task.map(AppMsg::Plugin), commands::init_command()]),
            )
        }
    };

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
        .title(APP_NAME)
        .run()
}
