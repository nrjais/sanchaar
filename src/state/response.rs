use crate::core::client;
use crate::state::TaskCancelKey;
use iced::widget::text_editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResponseTabId {
    #[default]
    Body,
    Headers,
}

#[derive(Debug)]
pub struct CompletedResponse {
    pub result: client::Response,
    pub content: text_editor::Content,
}

#[derive(Debug, Default)]
pub enum ResponseState {
    #[default]
    Idle,
    Executing(TaskCancelKey),
    Completed(CompletedResponse),
    Failed(anyhow::Error),
}

#[derive(Debug)]
pub struct ResponsePane {
    pub state: ResponseState,
    pub active_tab: ResponseTabId,
}

impl Default for ResponsePane {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponsePane {
    pub fn new() -> Self {
        Self {
            state: ResponseState::Idle,
            active_tab: ResponseTabId::Body,
        }
    }
}
