use core::persistence::collections::project_dirs;
use std::{fs, path::Path};

use anyhow::Context;
use iced::{Task, event::listen_with, window};

use crate::{
    app::AppMsg,
    state::{AppState, WindowState},
};

const WINDOW_STATE_FILE: &str = "window_state.json";

const DEFAULT_WINDOW_STATE: WindowState = WindowState {
    x: 0.0,
    y: 0.0,
    width: 1024.0,
    height: 768.0,
    changed: true,
};

#[derive(Debug, Clone)]
pub enum Message {
    Event(iced::Event),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        match self {
            Message::Event(event) => match event {
                iced::Event::Window(window::Event::Moved(point)) => {
                    state.window_state.x = point.x;
                    state.window_state.y = point.y;
                    state.window_state.changed = true;
                }
                iced::Event::Window(window::Event::Resized(size)) => {
                    state.window_state.width = size.width;
                    state.window_state.height = size.height;
                    state.window_state.changed = true;
                }
                _ => {}
            },
        }
        Task::none()
    }
}

pub async fn write_window_state(state: &WindowState) -> anyhow::Result<()> {
    let binding = project_dirs();
    let path = binding
        .as_ref()
        .map(|dirs| dirs.data_local_dir())
        .context("Failed to find data directory")?;
    let data = serde_json::to_string(state)?;
    tokio::fs::write(path.join(WINDOW_STATE_FILE), data).await?;
    Ok(())
}

fn read_window_state(path: &Path) -> anyhow::Result<WindowState> {
    let data = fs::read(path.join(WINDOW_STATE_FILE))?;
    Ok(serde_json::from_slice(&data)?)
}

pub fn load_window_state() -> (WindowState, bool) {
    project_dirs()
        .as_ref()
        .context("Failed to find data directory")
        .map(|dirs| dirs.data_local_dir())
        .and_then(read_window_state)
        .inspect_err(|e| log::error!("Error reading window state: {e:?}"))
        .map(|state| (state, false))
        .unwrap_or((DEFAULT_WINDOW_STATE, true))
}

pub fn subscription(_: &AppState) -> iced::Subscription<AppMsg> {
    listen_with(|event, _, _| match &event {
        iced::Event::Window(window::Event::Moved(_) | window::Event::Resized(_)) => Some(event),
        _ => None,
    })
    .map(Message::Event)
    .map(AppMsg::WindowEvent)
}
