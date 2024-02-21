use crate::state::response::ResponsePane;

use super::request::RequestPane;

#[derive(Debug)]
pub struct Tab {
    pub request: RequestPane,
    pub response: ResponsePane,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        Self {
            request: RequestPane::new(),
            response: ResponsePane::new(),
        }
    }
}
