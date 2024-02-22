pub mod request;
pub mod response;
pub mod tab;

use slotmap::SlotMap;
pub use tab::*;
use tokio::sync::oneshot;

use crate::{commands::Commands, core::client::create_client};

slotmap::new_key_type! {
    pub struct TaskCancelKey;
}

#[derive(Debug)]
pub struct AppCtx {
    pub client: reqwest::Client,
    pub task_cancel_tx: SlotMap<TaskCancelKey, oneshot::Sender<()>>,
}

impl AppCtx {
    pub fn cancel_task(&mut self, key: TaskCancelKey) {
        if let Some(tx) = self.task_cancel_tx.remove(key) {
            let _ = tx.send(());
        }
    }
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
                task_cancel_tx: SlotMap::with_key(),
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
