use iced::{Task, Theme};
use iced_auto_updater_plugin::{AutoUpdaterPlugin, ReleaseInfo, UpdaterConfig};
use iced_plugins::{PluginHandle, PluginManager, PluginManagerBuilder, PluginMessage};
use iced_window_state_plugin::WindowStatePlugin;
use indexmap::IndexMap;
use reqwest_cookie_store::CookieStoreRwLock;
use tabs::collection_tab::CollectionTab;
use tabs::cookies_tab::CookiesTab;
use tabs::history_tab::HistoryTab;
use tabs::perf_tab::PerfTab;

use core::client::{create_client, create_cookie_store};
use core::http::{CollectionRequest, Collections};
use core::persistence::history::HistoryDatabase;
use std::sync::Arc;
pub use tabs::http_tab::*;

use crate::APP_NAME;
use crate::app::AppMsg;
use crate::commands::JobState;
use crate::components::split::Direction;
use crate::state::popups::Popup;

pub mod environment;
pub mod popups;
pub mod request;
pub mod response;
pub mod session;
pub mod tabs;
pub mod utils;

core::new_id_type! {
    pub struct TabKey;
}

#[derive(Debug)]
pub enum Tab {
    Http(Box<HttpTab>),
    Collection(CollectionTab),
    CookieStore(CookiesTab),
    History(HistoryTab),
    Perf(Box<PerfTab>),
}

impl Tab {
    pub fn matches_type(&self, other: &Tab) -> bool {
        matches!(
            (self, other),
            (Tab::Http(_), Tab::Http(_))
                | (Tab::Collection(_), Tab::Collection(_))
                | (Tab::CookieStore(_), Tab::CookieStore(_))
                | (Tab::History(_), Tab::History(_))
                | (Tab::Perf(_), Tab::Perf(_))
        )
    }
}

#[derive(Debug)]
pub struct CommonState {
    pub collections: Collections,
    pub client: reqwest::Client,
    pub client_no_ssl: reqwest::Client,
    pub popup: Option<Popup>,
    pub background_tasks: Vec<JobState>,
    pub cookie_store: Arc<CookieStoreRwLock>,
    pub history_db: Option<HistoryDatabase>,
}

#[derive(Debug)]
pub struct PaneConfig {
    pub at: f32,
    pub side_bar_open: bool,
}

impl Default for PaneConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PaneConfig {
    pub fn new() -> Self {
        Self {
            at: 0.20,
            side_bar_open: true,
        }
    }

    pub fn toggle_side_bar(&mut self) {
        self.side_bar_open = !self.side_bar_open;
    }

    pub fn set_at(&mut self, at: f32) {
        self.at = at.clamp(0.20, 0.35);
    }
}

#[derive(Debug)]
pub struct Plugins {
    pub auto_updater: PluginHandle<AutoUpdaterPlugin>,
    pub manager: PluginManager,
}

#[derive(Debug, Clone)]
pub enum UpdateStatus {
    None,
    Available(ReleaseInfo),
    Downloading,
    Installing,
    Completed,
}

#[derive(Debug)]
pub struct MessageQueue {
    queue: Vec<AppMsg>,
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }
}

impl MessageQueue {
    pub fn push(&mut self, msg: AppMsg) {
        self.queue.push(msg);
    }

    pub fn task(&mut self) -> Task<AppMsg> {
        Task::batch(self.queue.drain(..).map(Task::done))
    }
}

#[derive(Debug)]
pub struct AppState {
    pub common: CommonState,
    pub plugins: Plugins,
    pub active_tab: TabKey,
    tab_history: indexmap::IndexSet<TabKey>,
    pub tabs: indexmap::IndexMap<TabKey, Tab>,
    pub pane_config: PaneConfig,
    pub split_direction: Direction,
    pub theme: Theme,
    pub update_status: UpdateStatus,
    pub queue: MessageQueue,
}

pub fn install_plugins() -> (Plugins, Task<PluginMessage>) {
    let auto_updater = UpdaterConfig {
        owner: "nrjais".to_string(),
        repo: "sanchaar".to_string(),
        current_version: env!("CARGO_PKG_VERSION").to_string(),
        auto_check_interval: 24 * 60 * 60,
        check_on_start: true,
    };

    let mut builder =
        PluginManagerBuilder::new().with_plugin(WindowStatePlugin::new(APP_NAME.to_string()));
    let auto_updater = builder.install(AutoUpdaterPlugin::new(APP_NAME.to_string(), auto_updater));

    let (manager, task) = builder.build();

    (
        Plugins {
            auto_updater,
            manager,
        },
        task,
    )
}

impl AppState {
    pub fn new(plugins: Plugins) -> Self {
        let store = create_cookie_store();
        Self {
            plugins,
            active_tab: TabKey::ZERO,
            tabs: IndexMap::new(),
            tab_history: indexmap::IndexSet::new(),
            common: CommonState {
                client: create_client(false, Arc::clone(&store)),
                client_no_ssl: create_client(true, Arc::clone(&store)),
                cookie_store: store,
                collections: Collections::default(),
                popup: None,
                background_tasks: Vec::new(),
                history_db: None,
            },
            pane_config: PaneConfig::new(),
            split_direction: Direction::Horizontal,
            theme: Theme::GruvboxDark,
            update_status: UpdateStatus::None,
            queue: MessageQueue::new(),
        }
    }

    pub fn switch_tab(&mut self, tab: TabKey) {
        self.active_tab = tab;
        self.tab_history.shift_remove(&tab);
        self.tab_history.insert(tab);
    }

    pub fn open_unique_tab(&mut self, tab: Tab) {
        let existing_tab = self
            .tabs
            .iter()
            .find(|(_, t)| t.matches_type(&tab))
            .map(|(key, _)| *key);

        if let Some(key) = existing_tab {
            self.switch_tab(key);
        } else {
            self.open_tab(tab);
        }
    }

    pub fn open_tab(&mut self, tab: Tab) {
        let id = TabKey::new();
        self.tabs.insert(id, tab);
        self.switch_tab(id);
    }

    pub fn switch_to_tab(&mut self, req: CollectionRequest) -> bool {
        self.tabs
            .iter()
            .filter_map(|(key, tab)| match tab {
                Tab::Http(tab) => Some((key, tab)),
                _ => None,
            })
            .find(|(_, tab)| tab.collection_ref == req)
            .map(|(key, _)| *key)
            .inspect(|tab| {
                self.switch_tab(*tab);
            })
            .is_some()
    }

    pub fn get_tab_mut(&mut self, key: TabKey) -> Option<&mut Tab> {
        self.tabs.get_mut(&key)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(&self.active_tab)
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(&self.active_tab)
    }

    pub fn close_tab(&mut self, tab: TabKey) {
        self.tabs.shift_remove(&tab);
        let mut tab = self.tab_history.pop();
        while let Some(key) = tab {
            if self.tabs.contains_key(&key) {
                self.switch_tab(key);
                break;
            }

            tab = self.tab_history.pop();
        }
    }

    pub fn reorder_tab(&mut self, dragged_tab: TabKey, target_tab: TabKey) {
        let dragged_index = self.tabs.get_index_of(&dragged_tab);
        let target_index = self.tabs.get_index_of(&target_tab);

        if let Some((from_idx, to_idx)) = dragged_index.zip(target_index) {
            self.tabs.move_index(from_idx, to_idx);
        }
    }

    pub(crate) fn close_all_tabs(&mut self) {
        self.tabs.clear();
        self.active_tab = TabKey::ZERO;
        self.tab_history.clear();
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
