use iced::Command;
use log::info;

use core::http::collection::Collection;
use core::persistence::collections;

use crate::{app::AppMsg, state::TabKey, AppState};

use self::builders::{check_dirty_requests_cmd, load_collections_cmd};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundTasks {
    SaveCollections,
    CheckDirtyRequests,
}

fn remove_task(state: &mut AppState, task: BackgroundTasks) {
    state.background_tasks.retain(|t| t != &task);
}

#[derive(Debug, Clone)]
pub enum CommandMsg {
    CollectionsLoaded(Vec<Collection>),
    Completed(String),
    UpdateDirtyTabs(Vec<TabKey>),
}

impl CommandMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CommandMsg::CollectionsLoaded(collection) => {
                state.collections.insert_all(collection);
                remove_task(state, BackgroundTasks::SaveCollections);
            }
            CommandMsg::Completed(msg) => {
                info!("Command completed: {}", msg);
            }
            CommandMsg::UpdateDirtyTabs(dirty) => {
                for key in dirty {
                    if let Some(tab) = state.tabs.get_mut(key) {
                        tab.mark_request_dirty();
                    };
                }
                remove_task(state, BackgroundTasks::CheckDirtyRequests);
            }
        };
        Command::none()
    }
}

fn save_open_collections(state: &mut AppState) -> Command<CommandMsg> {
    let task = BackgroundTasks::SaveCollections;
    if !state.collections.dirty || state.background_tasks.contains(&task) {
        return Command::none();
    }

    state.background_tasks.push(task);

    let collections = state.collections.get_collections_for_save();
    Command::perform(collections::save(collections), |result| match result {
        Ok(_) => CommandMsg::Completed("Collections saved".to_string()),
        Err(e) => CommandMsg::Completed(format!("Error saving collections: {:?}", e)),
    })
}

fn check_dirty_requests(state: &mut AppState) -> Command<CommandMsg> {
    let task = BackgroundTasks::CheckDirtyRequests;
    if state.background_tasks.contains(&task) {
        return Command::none();
    }

    let (cmd, exec) = check_dirty_requests_cmd(state, CommandMsg::UpdateDirtyTabs);
    if exec {
        state.background_tasks.push(task);
    }
    cmd
}

pub fn background(state: &mut AppState) -> Command<CommandMsg> {
    Command::batch([save_open_collections(state), check_dirty_requests(state)])
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections_cmd(), CommandMsg::CollectionsLoaded).map(AppMsg::Command)
}
