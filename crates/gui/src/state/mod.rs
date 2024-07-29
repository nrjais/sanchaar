use collection_tab::CollectionTab;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use iced::Theme;
use indexmap::IndexMap;

use core::client::create_client;
use core::http::{CollectionRequest, Collections};
pub use http_tab::*;

use crate::commands::JobState;
use crate::state::popups::Popup;
use crate::state::response::ResponseState;

pub mod collection_tab;
pub mod environment;
pub mod http_tab;
pub mod popups;
pub mod request;
pub mod response;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitState {
    First,
    // Left or Top
    Second, // Right or Bottom
}

core::new_id_type! {
    pub struct TabKey;
}

#[derive(Debug)]
pub enum Tab {
    Http(HttpTab),
    Collection(CollectionTab),
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: Option<TabKey>,
    tab_history: indexmap::IndexSet<TabKey>,
    pub tabs: indexmap::IndexMap<TabKey, Tab>,
    pub collections: Collections,
    pub client: reqwest::Client,
    pub client_no_ssl: reqwest::Client,
    pub panes: pane_grid::State<SplitState>,
    pub popup: Option<Popup>,
    pub theme: Theme,
    pub background_tasks: Vec<JobState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_tab: None,
            tabs: IndexMap::new(),
            tab_history: indexmap::IndexSet::new(),
            client: create_client(false),
            client_no_ssl: create_client(true),
            collections: Collections::default(),
            panes: pane_grid::State::with_configuration(Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.15,
                a: Box::new(Configuration::Pane(SplitState::First)),
                b: Box::new(Configuration::Pane(SplitState::Second)),
            }),
            popup: None,
            theme: Theme::GruvboxDark,
            background_tasks: Vec::new(),
        }
    }

    pub fn switch_tab(&mut self, tab: TabKey) {
        self.active_tab = Some(tab);
        self.tab_history.shift_remove(&tab);
        self.tab_history.insert(tab);
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

    pub fn get_tab(&self, key: TabKey) -> Option<&Tab> {
        self.tabs.get(&key)
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(&self.active_tab?)
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.get(&self.active_tab?)
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
        self.active_tab = None;
        self.tab_history.clear();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
