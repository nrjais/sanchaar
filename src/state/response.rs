use crate::core::client;
use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResponseTabId {
    #[default]
    Body,
    Headers,
}

#[derive(Debug)]
pub struct Response {
    pub response: Option<client::Response>,
    pub text_viewer: text_editor::Content,
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
            text_viewer: text_editor::Content::default(),
        }
    }
}
