use core::http::collection::Collection;
use core::persistence::collections;
use core::persistence::history::{HistoryDatabase, HistoryEntrySummary, get_history_db_path};
use iced::Task;
use log::info;
use std::time::Instant;

use crate::{
    app::AppMsg,
    state::{AppState, RequestDirtyState, Tab, TabKey},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundTask {
    SaveCollections,
    SaveEnvironments,
    CheckDirtyRequests,
    InitializeHistory,
    LoadHistory,
    SearchHistory,
}

fn remove_task(state: &mut AppState, task: BackgroundTask) {
    state.common.background_tasks.retain(|t| t.task != task);
}

fn task_done(state: &mut AppState, task: BackgroundTask) {
    if let Some(job) = state
        .common
        .background_tasks
        .iter_mut()
        .find(|t| t.task == task)
    {
        job.done = true;
    }
}

fn schedule_task(state: &mut AppState, task: BackgroundTask, delay: u64) -> bool {
    let job = state
        .common
        .background_tasks
        .iter()
        .find(|t| t.task == task);

    let sch = match job {
        Some(job) => job.started.elapsed().as_secs() > delay && job.done,
        None => true,
    };
    if sch {
        remove_task(state, task);
        state.common.background_tasks.push(JobState {
            task,
            done: false,
            started: Instant::now(),
        });
    }
    sch
}

#[derive(Debug, Clone)]
pub enum TaskMsg {
    CollectionsLoaded(Vec<Collection>),
    Completed(BackgroundTask),
    UpdateDirtyTabs(Vec<(TabKey, RequestDirtyState)>),
    HistoryInitialized(Option<HistoryDatabase>),
    HistoryLoaded(Vec<HistoryEntrySummary>),
    SearchHistoryCompleted(Vec<HistoryEntrySummary>),
}

impl TaskMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            TaskMsg::CollectionsLoaded(collection) => {
                state.common.collections.insert_all(collection);
                task_done(state, BackgroundTask::SaveCollections);
            }
            TaskMsg::Completed(task) => {
                task_done(state, task);
            }
            TaskMsg::UpdateDirtyTabs(status) => {
                task_done(state, BackgroundTask::CheckDirtyRequests);
                for (key, status) in status {
                    if let Some(Tab::Http(tab)) = state.tabs.get_mut(&key) {
                        tab.request_dirty_state = status;
                    };
                }
            }
            TaskMsg::HistoryInitialized(db) => {
                task_done(state, BackgroundTask::InitializeHistory);
                state.common.history_db = db;
            }
            TaskMsg::HistoryLoaded(entries) => {
                task_done(state, BackgroundTask::LoadHistory);
                for tab in state.tabs.values_mut() {
                    if let Tab::History(history_tab) = tab {
                        history_tab.set_entries(entries.clone());
                        history_tab.set_searching(false);
                    }
                }
            }
            TaskMsg::SearchHistoryCompleted(entries) => {
                task_done(state, BackgroundTask::SearchHistory);
                for tab in state.tabs.values_mut() {
                    if let Tab::History(history_tab) = tab {
                        history_tab.set_entries(entries.clone());
                        history_tab.set_searching(false);
                    }
                }
            }
        };
        Task::none()
    }
}

fn save_open_collections(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveCollections;
    let schedule = state.common.collections.dirty && schedule_task(state, task, 1);
    if !schedule {
        return Task::none();
    }

    let collections = state.common.collections.get_collections_for_save();
    Task::perform(collections::save(collections), |result| match result {
        Ok(_) => TaskMsg::Completed(BackgroundTask::SaveCollections),
        Err(e) => {
            log::error!("Error saving collections: {e:?}");
            TaskMsg::Completed(BackgroundTask::SaveCollections)
        }
    })
}

fn save_environments(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveEnvironments;
    let schedule = state.common.collections.dirty && schedule_task(state, task, 1);
    if !schedule {
        return Task::none();
    }

    let mut environments = Vec::new();
    for collection in state.tabs.values_mut() {
        if let Tab::Collection(tab) = collection
            && tab.env_editor.edited
        {
            let envs = tab.env_editor.get_envs_for_save();
            environments.push((tab.collection_key, envs));
        }
    }

    let mut tasks = Vec::new();
    for (key, envs) in environments {
        let task = state
            .common
            .collections
            .with_collection_mut(key, |c| builders::save_environments_cmd(c, envs));
        tasks.push(task.unwrap_or(Task::none()));
    }

    Task::batch(tasks).map(|_| TaskMsg::Completed(BackgroundTask::SaveEnvironments))
}

fn check_dirty_requests(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::CheckDirtyRequests;
    if !schedule_task(state, task, 2) {
        return Task::none();
    }

    check_dirty_requests_cmd(state).map(TaskMsg::UpdateDirtyTabs)
}

fn load_history(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::LoadHistory;

    let history_tab_open = state
        .active_tab()
        .map(|tab| matches!(tab, Tab::History(_)))
        .unwrap_or(false);

    if !history_tab_open {
        return Task::none();
    }

    if let Some(Tab::History(tab)) = state.active_tab()
        && !tab.search_query_text.is_empty()
    {
        return Task::none();
    }

    #[cfg(not(feature = "default"))]
    let delay = 5;
    #[cfg(feature = "default")]
    let delay = 500000;

    if !schedule_task(state, task, delay) {
        return Task::none();
    }

    let Some(history_db) = state.common.history_db.clone() else {
        return Task::none();
    };

    Task::future(async move {
        match history_db.get_history_summary(Some(100)).await {
            Ok(entries) => TaskMsg::HistoryLoaded(entries),
            Err(e) => {
                log::error!("Error loading history: {e:?}");
                TaskMsg::Completed(BackgroundTask::LoadHistory)
            }
        }
    })
}

fn search_history(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SearchHistory;

    let history_tab_open = state
        .active_tab()
        .map(|tab| matches!(tab, Tab::History(_)))
        .unwrap_or(false);

    if !history_tab_open {
        return Task::none();
    }

    let (search_query, should_search) = if let Some(Tab::History(tab)) = state.active_tab_mut() {
        let should_search = tab.should_trigger_search() && !tab.search_query_text.is_empty();
        if should_search {
            tab.clear_search_timer();
            tab.set_searching(true);
        }
        (tab.search_query_text.clone(), should_search)
    } else {
        return Task::none();
    };

    if !should_search || !schedule_task(state, task, 0) {
        return Task::none();
    }

    let Some(history_db) = state.common.history_db.clone() else {
        return Task::none();
    };

    Task::future(async move {
        match history_db
            .search_history_summary(&search_query, Some(100))
            .await
        {
            Ok(entries) => TaskMsg::SearchHistoryCompleted(entries),
            Err(e) => {
                log::error!("Error searching history: {e:?}");
                TaskMsg::Completed(BackgroundTask::SearchHistory)
            }
        }
    })
}

pub fn background(state: &mut AppState) -> Task<TaskMsg> {
    Task::batch([
        save_open_collections(state),
        save_environments(state),
        check_dirty_requests(state),
        load_history(state),
        search_history(state),
    ])
}

pub async fn init_history_db_cmd() -> Option<HistoryDatabase> {
    match get_history_db_path() {
        Ok(path) => match HistoryDatabase::new(path).await {
            Ok(db) => {
                info!("History database initialized successfully");
                Some(db)
            }
            Err(e) => {
                eprintln!("Failed to initialize history database: {e}");
                None
            }
        },
        Err(e) => {
            eprintln!("Failed to get history database path: {e}");
            None
        }
    }
}

pub fn init_command() -> Task<AppMsg> {
    Task::batch([
        Task::perform(load_collections_cmd(), TaskMsg::CollectionsLoaded).map(AppMsg::Command),
        Task::perform(init_history_db_cmd(), TaskMsg::HistoryInitialized).map(AppMsg::Command),
    ])
}
