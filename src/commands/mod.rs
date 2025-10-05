use core::http::collection::Collection;
use core::persistence::collections;
use core::persistence::history::{HistoryDatabase, HistoryEntrySummary, get_history_db_path};
use iced::Task;
use log::info;
use std::time::Instant;

use crate::state::session::{self, SessionState, load_session_state};
use crate::window::write_window_state;
use crate::{
    app::AppMsg,
    state::{AppState, RequestDirtyState, Tab, TabKey},
};

use self::builders::{check_dirty_requests_cmd, load_collections_cmd};

pub mod builders;
mod cancellable_task;
pub mod dialog;

#[cfg(not(feature = "default"))]
const DELAY: u64 = 1;
#[cfg(feature = "default")]
const DELAY: u64 = 100000;

#[derive(Debug, Clone)]
pub struct JobState {
    task: BackgroundTask,
    done: bool,
    started: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundTask {
    SaveOpenCollections,
    SaveCollections,
    SaveEnvironments,
    CheckDirtyRequests,
    InitializeHistory,
    LoadHistory,
    SearchHistory,
    SaveWindowState,
    SaveSessionState,
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
    SessionLoaded(Option<SessionState>),
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
            TaskMsg::SessionLoaded(session) => {
                if let Some(session) = session {
                    state.restore_session(session)
                }
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
    let task = BackgroundTask::SaveOpenCollections;
    let schedule = state.common.collections.dirty && schedule_task(state, task, DELAY);
    if !schedule {
        return Task::none();
    }

    let collections = state.common.collections.get_collections_for_save();
    Task::perform(collections::save(collections), |result| match result {
        Ok(_) => TaskMsg::Completed(BackgroundTask::SaveOpenCollections),
        Err(e) => {
            log::error!("Error saving collections: {e:?}");
            TaskMsg::Completed(BackgroundTask::SaveOpenCollections)
        }
    })
}

fn save_environments(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveEnvironments;
    if !schedule_task(state, task, DELAY) {
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
    if !schedule_task(state, task, DELAY * 2) {
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

    if !schedule_task(state, task, DELAY * 5) {
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

fn save_window_state(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveWindowState;
    if !schedule_task(state, task, DELAY) {
        return Task::none();
    }

    let window_state = state.window_state.clone();
    Task::future(async move {
        match write_window_state(&window_state).await {
            Ok(_) => TaskMsg::Completed(BackgroundTask::SaveWindowState),
            Err(e) => {
                log::error!("Error saving window state: {e:?}");
                TaskMsg::Completed(BackgroundTask::SaveWindowState)
            }
        }
    })
}

fn save_collections(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveCollections;
    let schedule = state.common.collections.dirty && schedule_task(state, task, DELAY);
    if !schedule {
        return Task::none();
    }

    let mut tasks = Vec::new();
    for collection in state.tabs.values_mut() {
        if let Tab::Collection(tab) = collection
            && tab.edited
        {
            let task = state
                .common
                .collections
                .get_mut(tab.collection_key)
                .map(|c| builders::save_collection_cmd(c, tab));
            tasks.push(task.unwrap_or(Task::none()));
        }
    }

    Task::batch(tasks).map(|_| TaskMsg::Completed(BackgroundTask::SaveCollections))
}

fn save_session_state(state: &mut AppState) -> Task<TaskMsg> {
    let task = BackgroundTask::SaveSessionState;
    if !schedule_task(state, task, DELAY) {
        return Task::none();
    }

    println!("Saving session state");
    let session_state = SessionState::from_app_state(state);
    Task::future(async move {
        match session::save_session_state(&session_state).await {
            Ok(_) => TaskMsg::Completed(BackgroundTask::SaveSessionState),
            Err(e) => {
                log::error!("Error saving session state: {e:?}");
                TaskMsg::Completed(BackgroundTask::SaveSessionState)
            }
        }
    })
}

pub fn background(state: &mut AppState) -> Task<TaskMsg> {
    Task::batch([
        save_open_collections(state),
        save_environments(state),
        check_dirty_requests(state),
        save_session_state(state),
        load_history(state),
        search_history(state),
        save_collections(state),
        save_window_state(state),
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

pub async fn load_session_state_cmd() -> Option<SessionState> {
    match load_session_state().await {
        Ok(session) => {
            info!("Session state loaded successfully");
            Some(session)
        }
        Err(e) => {
            log::info!("No session state found or error loading: {e:?}");
            None
        }
    }
}

pub fn init_command() -> Task<AppMsg> {
    Task::batch([
        Task::perform(load_collections_cmd(), TaskMsg::CollectionsLoaded)
            .chain(Task::perform(
                load_session_state_cmd(),
                TaskMsg::SessionLoaded,
            ))
            .map(AppMsg::Command),
        Task::perform(init_history_db_cmd(), TaskMsg::HistoryInitialized).map(AppMsg::Command),
    ])
}
