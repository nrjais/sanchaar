pub mod request;
pub mod response;
pub mod tab;

use slotmap::SlotMap;
pub use tab::*;

use crate::commands::AppCommand;
use crate::state::response::ResponseState;
use crate::{commands::Commands, core::client::create_client};

slotmap::new_key_type! {
    pub struct TabKey;
}

#[derive(Debug)]
pub struct AppCtx {
    pub client: reqwest::Client,
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: TabKey,
    pub tabs: SlotMap<TabKey, Tab>,
    pub ctx: AppCtx,
    pub commands: Commands,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        let tab = Tab::new();
        let mut tabs = SlotMap::with_key();
        let active_tab = tabs.insert(tab);

        Self {
            active_tab,
            tabs,
            ctx: AppCtx {
                client: create_client(),
            },
            commands: Commands::new(),
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
        if self.active_tab == tab {
            self.active_tab = self.tabs.insert(Tab::new());
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
