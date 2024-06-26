use iced::widget::pane_grid;
use iced::widget::pane_grid::Configuration;
use iced::Theme;
use slotmap::SlotMap;

use core::client::create_client;
use core::http::collection::RequestRef;
use core::http::request::Request;
use core::http::{CollectionRequest, Collections};
pub use tab::*;

use crate::commands::JobState;
use crate::state::popups::Popup;
use crate::state::response::ResponseState;

pub mod environment;
pub mod popups;
pub mod request;
pub mod response;
pub mod tab;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitState {
    First,
    // Left or Top
    Second, // Right or Bottom
}

slotmap::new_key_type! {
    pub struct TabKey;
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: TabKey,
    pub tabs: SlotMap<TabKey, Tab>,
    pub collections: Collections,
    pub client: reqwest::Client,
    pub panes: pane_grid::State<SplitState>,
    pub popup: Option<Popup>,
    pub theme: Theme,
    pub background_tasks: Vec<JobState>,
}

impl AppState {
    pub fn new() -> Self {
        let tab = Tab::default();
        let mut tabs = SlotMap::with_key();
        let active_tab = tabs.insert(tab);

        Self {
            active_tab,
            tabs,
            client: create_client(),
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

    pub fn open_request(&mut self, req_ref: CollectionRequest, request: Request) {
        self.active_tab = self.tabs.insert(Tab::with_ref(request, req_ref));
    }

    pub fn switch_to_tab(&mut self, req: CollectionRequest) -> bool {
        self.tabs
            .iter()
            .find(|tab| tab.1.collection_ref == Some(req))
            .inspect(|tab| {
                self.active_tab = tab.0;
            })
            .is_some()
    }

    pub fn get_tab_mut(&mut self, key: TabKey) -> Option<&mut Tab> {
        self.tabs.get_mut(key)
    }

    pub fn get_tab(&self, key: TabKey) -> Option<&Tab> {
        self.tabs.get(key)
    }

    pub fn active_tab_mut(&mut self) -> &mut Tab {
        self.tabs
            .get_mut(self.active_tab)
            .expect("Active tab not found")
    }

    pub fn active_tab(&self) -> &Tab {
        self.tabs
            .get(self.active_tab)
            .expect("Active tab not found")
    }

    pub fn clear_tab_tasks(&mut self, tab: TabKey) {
        if let Some(tab) = self.get_tab_mut(tab) {
            tab.cancel_tasks();
        }
    }

    pub fn cancel_tab_tasks(&mut self, tab: TabKey) {
        if let Some(tab) = self.get_tab_mut(tab) {
            tab.cancel_tasks();
        }

        self.active_tab_mut().response.state = ResponseState::Idle;
    }

    pub fn close_tab(&mut self, tab: TabKey) {
        self.tabs.remove(tab);
        self.active_tab = self
            .tabs
            .keys()
            .next()
            .unwrap_or_else(|| self.tabs.insert(Default::default()));
    }

    pub(crate) fn get_req_ref(&self, tab: TabKey) -> Option<&RequestRef> {
        let tab = self.tabs.get(tab)?;
        let req_ref = tab.collection_ref.as_ref()?;
        self.collections.get_ref(*req_ref)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
