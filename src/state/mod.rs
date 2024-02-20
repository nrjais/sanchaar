pub mod request;
pub mod response;
pub mod tab;

pub use tab::*;

use crate::{commands::Commands, core::client::create_client};

#[derive(Debug)]
pub struct AppCtx {
    pub client: reqwest::Client,
}

#[derive(Debug)]
pub struct AppState {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
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
        let tabs = vec![Tab::new()];
        Self {
            active_tab: 0,
            tabs,
            ctx: AppCtx {
                client: create_client(),
            },
            commands: Commands::new(),
        }
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
}
