use slotmap::SlotMap;

pub use tab::*;

use crate::commands::AppCommand;
use crate::state::collection::Collection;
use crate::state::response::ResponseState;
use crate::{commands::Commands, core::client::create_client};

pub mod collection;
pub mod request;
pub mod response;
pub mod tab;

slotmap::new_key_type! {
    pub struct TabKey;
    pub struct CollectionKey;
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: TabKey,
    pub tabs: SlotMap<TabKey, Tab>,
    pub collections: SlotMap<CollectionKey, Collection>,
    pub commands: Commands,
    pub client: reqwest::Client,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn test_collection() -> SlotMap<CollectionKey, Collection> {
    let mut collections = SlotMap::with_key();

    let collection = Collection {
        name: "Test Collection".to_string(),
        children: vec![
            collection::Entry::Item(collection::Item {
                name: "Item 1".to_string(),
            }),
            collection::Entry::Folder(collection::Folder {
                name: "Folder 1".to_string(),
                children: vec![collection::Entry::Item(collection::Item {
                    name: "Item 2".to_string(),
                })],
                expanded: true,
            }),
        ],
        expanded: true,
    };
    let _ = collections.insert(collection);

    collections
}

impl AppState {
    pub fn new() -> Self {
        let tab = Tab::new();
        let mut tabs = SlotMap::with_key();
        let active_tab = tabs.insert(tab);

        Self {
            active_tab,
            tabs,
            client: create_client(),
            commands: Commands::new(),
            collections: test_collection(),
        }
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
        if self.tabs.is_empty() {
            self.active_tab = self.tabs.insert(Default::default());
        } else if self.active_tab == tab {
            self.active_tab = self.tabs.keys().next().unwrap();
        }
    }

    pub fn send_request(&mut self) {
        let active_tab = self.active_tab_mut();
        if let ResponseState::Executing = active_tab.response.state {
            self.cancel_tab_tasks(self.active_tab);
        }

        self.commands.add(AppCommand::InitRequest(self.active_tab));
    }

    pub fn save_request(&mut self) {
        self.commands.add(AppCommand::SaveRequest(self.active_tab));
    }
}
