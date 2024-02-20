use crate::core::client;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResponseTabId {
    #[default]
    Body,
    Headers,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub response: Option<client::Response>,
    pub active_tab: ResponseTabId,
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            response: None,
            active_tab: ResponseTabId::Body,
        }
    }
}
