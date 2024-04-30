use iced::Command;
use log::info;

use core::http::collection::Collection;
use core::persistence::collections;

use crate::{app::AppMsg, AppState};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug, Clone)]
pub enum CommandMsg {
    CollectionsLoaded(Vec<Collection>),
    Completed(String),
}

impl CommandMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CommandMsg::CollectionsLoaded(collection) => {
                state.collections.insert_all(collection);
            }
            CommandMsg::Completed(msg) => {
                info!("Command completed: {}", msg);
            }
        };
        Command::none()
    }
}

fn save_open_collections(state: &AppState) -> Command<CommandMsg> {
    if !state.collections.dirty {
        return Command::none();
    }

    let collections = state.collections.entries.values().cloned().collect();
    Command::perform(collections::save(collections), |result| match result {
        Ok(_) => CommandMsg::Completed("Collections saved".to_string()),
        Err(e) => CommandMsg::Completed(format!("Error saving collections: {:?}", e)),
    })
}

pub fn background(state: &mut AppState) -> Command<CommandMsg> {
    Command::batch([save_open_collections(state)])
}

pub async fn load_collections() -> Vec<Collection> {
    collections::load().await.unwrap_or_else(|e| {
        println!("Error loading http: {:?}", e);
        vec![]
    })
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections(), CommandMsg::CollectionsLoaded).map(AppMsg::Command)
}
