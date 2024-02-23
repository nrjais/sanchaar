pub mod request;
pub mod response;
pub mod tab;

use slotmap::SlotMap;
pub use tab::*;
use tokio::sync::oneshot;

use crate::{commands::Commands, core::client::create_client};

slotmap::new_key_type! {
    pub struct TaskCancelKey;
    pub struct TabKey;
}

#[derive(Debug)]
pub struct AppCtx {
    pub client: reqwest::Client,
    pub task_cancel_tx: SlotMap<TaskCancelKey, oneshot::Sender<()>>,
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

    pub fn cancel_task(&mut self, key: TaskCancelKey) {
        if let Some(tx) = self.ctx.task_cancel_tx.remove(key) {
            let _ = tx.send(());
        }

        self.active_tab_mut().response.state = response::ResponseState::Idle;
    }
}
