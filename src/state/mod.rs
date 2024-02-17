use strum::{Display, EnumString, VariantArray};

#[derive(Debug)]
pub struct AppState {
    pub active_tab: usize,
    pub tabs: Vec<Tab>,
}

impl AppState {
    pub fn new() -> Self {
        let tabs = vec![Tab::default()];
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

#[derive(Debug, Clone)]
pub struct Tab {
    pub url: String,
    pub method: Method,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            url: "http://echo.nrjais.com".to_string(),
            method: Method::GET,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, VariantArray, Display, Default)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}
