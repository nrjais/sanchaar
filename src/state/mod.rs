pub mod request;
pub mod tab;

pub use tab::*;

#[derive(Debug)]
pub struct AppState {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

impl AppState {
    pub fn new() -> Self {
        let tabs = vec![Tab::new()];
        Self {
            active_tab: 0,
            tabs,
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
