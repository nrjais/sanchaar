use iced::Command;
use log::info;

use core::http::collection::Collection;
use core::persistence::collections;
use std::time::Instant;

use crate::{
    app::AppMsg,
    state::{RequestDirtyState, TabKey},
    AppState,
};

use self::builders::{check_dirty_requests_cmd, load_collections_cmd};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[derive(Debug, Clone)]
pub struct JobState {
    task: BackgroundTask,
    done: bool,
    started: Instant,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BackgroundTask {
    SaveCollections,
    CheckDirtyRequests,
}

fn remove_task(state: &mut AppState, task: BackgroundTask) {
    state.background_tasks.retain(|t| t.task != task);
}

fn task_done(state: &mut AppState, task: BackgroundTask) {
    if let Some(job) = state.background_tasks.iter_mut().find(|t| t.task == task) {
        job.done = true;
    }
}

fn schedule_task(state: &mut AppState, task: BackgroundTask, delay: u64) -> bool {
    let job = state.background_tasks.iter().find(|t| t.task == task);

    let sch = match job {
        Some(job) => job.started.elapsed().as_secs() > delay && job.done,
        None => true,
    };
    if sch {
        remove_task(state, task);
        state.background_tasks.push(JobState {
            task,
            done: false,
            started: Instant::now(),
        });
    }
    sch
}

#[derive(Debug, Clone)]
pub enum CommandMsg {
    CollectionsLoaded(Vec<Collection>),
    Completed(String),
    UpdateDirtyTabs(Vec<(TabKey, RequestDirtyState)>),
}

impl CommandMsg {
    pub fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            CommandMsg::CollectionsLoaded(collection) => {
                task_done(state, BackgroundTask::SaveCollections);
                state.collections.insert_all(collection);
            }
            CommandMsg::Completed(msg) => {
                info!("Command completed: {}", msg);
            }
            CommandMsg::UpdateDirtyTabs(status) => {
                task_done(state, BackgroundTask::CheckDirtyRequests);
                for (key, status) in status {
                    if let Some(tab) = state.tabs.get_mut(key) {
                        tab.request_dirty_state = status;
                    };
                }
            }
        };
        Command::none()
    }
}

fn save_open_collections(state: &mut AppState) -> Command<CommandMsg> {
    let task = BackgroundTask::SaveCollections;
    if !state.collections.dirty || !schedule_task(state, task, 0) {
        return Command::none();
    }

    let collections = state.collections.get_collections_for_save();
    Command::perform(collections::save(collections), |result| match result {
        Ok(_) => CommandMsg::Completed("Collections saved".to_string()),
        Err(e) => CommandMsg::Completed(format!("Error saving collections: {:?}", e)),
    })
}

fn check_dirty_requests(state: &mut AppState) -> Command<CommandMsg> {
    let task = BackgroundTask::CheckDirtyRequests;
    if !schedule_task(state, task, 5) {
        return Command::none();
    }

    check_dirty_requests_cmd(state, CommandMsg::UpdateDirtyTabs)
}

pub fn background(state: &mut AppState) -> Command<CommandMsg> {
    Command::batch([save_open_collections(state), check_dirty_requests(state)])
}

pub fn init_command() -> Command<AppMsg> {
    Command::perform(load_collections_cmd(), CommandMsg::CollectionsLoaded).map(AppMsg::Command)
}
