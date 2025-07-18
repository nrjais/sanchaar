use iced::Theme;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use indexmap::IndexMap;
use reqwest_cookie_store::CookieStoreRwLock;
use tabs::collection_tab::CollectionTab;
use tabs::cookies_tab::CookiesTab;
use tabs::history_tab::HistoryTab;

use core::client::{create_client, create_cookie_store};
use core::http::{CollectionRequest, Collections};
use core::persistence::history::HistoryDatabase;
use std::sync::Arc;
pub use tabs::http_tab::*;

use crate::commands::JobState;
use crate::state::popups::Popup;
use crate::state::response::ResponseState;

pub mod environment;
pub mod popups;
pub mod request;
pub mod response;
pub mod tabs;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitState {
    // Left or Top
    First,
    // Right or Bottom
    Second,
}

core::new_id_type! {
    pub struct TabKey;
}

#[derive(Debug)]
pub enum Tab {
    Http(Box<HttpTab>),
    Collection(CollectionTab),
    CookieStore(CookiesTab),
    History(HistoryTab),
}

impl Tab {
    pub fn matches_type(&self, other: &Tab) -> bool {
        match (self, other) {
            (Tab::Http(_), Tab::Http(_)) => true,
            (Tab::Collection(_), Tab::Collection(_)) => true,
            (Tab::CookieStore(_), Tab::CookieStore(_)) => true,
            (Tab::History(_), Tab::History(_)) => true,
            _ => false,
        }
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
pub struct AppState {
    pub common: CommonState,
    pub active_tab: TabKey,
    tab_history: indexmap::IndexSet<TabKey>,
    pub tabs: indexmap::IndexMap<TabKey, Tab>,
    pub panes: pane_grid::State<SplitState>,
    pub theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let store = create_cookie_store();
        Self {
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
            panes: pane_grid::State::with_configuration(Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.20,
                a: Box::new(Configuration::Pane(SplitState::First)),
                b: Box::new(Configuration::Pane(SplitState::Second)),
            }),
            theme: Theme::GruvboxDark,
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

    pub fn cancel_tab_tasks(&mut self, tab: TabKey) {
        if let Some(Tab::Http(tab)) = self.get_tab_mut(tab) {
            tab.cancel_tasks();
            tab.response.state = ResponseState::Idle;
        }
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

    pub(crate) fn close_all_tabs(&mut self) {
        self.tabs.clear();
        self.active_tab = TabKey::ZERO;
        self.tab_history.clear();
    }
}
