use std::mem;

use iced::Command;

use core::http::collection::Collection;
use core::persistence::collections;

use crate::{app::AppMsg, AppState};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug)]
pub enum AppCommand {}

#[derive(Debug, Clone)]
pub enum CommandResultMsg {
    CollectionsLoaded(Vec<Collection>),
}

impl CommandResultMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CommandResultMsg::CollectionsLoaded(collection) => {
                state.collections.insert_all(collection);
            }
        };
        Command::none()
    }
}

#[derive(Debug)]
pub struct Commands(Vec<AppCommand>);

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

impl Commands {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn take(&mut self) -> Vec<AppCommand> {
        mem::take(&mut self.0)
    }

    pub fn add(&mut self, cmd: AppCommand) {
        self.0.push(cmd);
    }
}

pub async fn load_collections() -> Vec<Collection> {
    collections::load().await.unwrap_or_else(|e| {
        println!("Error loading http: {:?}", e);
        vec![]
    })
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections(), CommandResultMsg::CollectionsLoaded).map(AppMsg::Command)
}
