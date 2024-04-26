use components::text_editor;
use core::client;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResponseTabId {
    #[default]
    Body,
    Headers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BodyMode {
    Pretty,
    Raw,
}

#[derive(Debug)]
pub struct CompletedResponse {
    pub result: client::Response,
    pub content: Option<text_editor::Content>,
    pub raw: text_editor::Content,
    pub mode: BodyMode,
}

impl CompletedResponse {
    pub fn selected_content(&self) -> &text_editor::Content {
        match self.mode {
            BodyMode::Pretty => self.content.as_ref().unwrap_or(&self.raw),
            BodyMode::Raw => &self.raw,
        }
    }
    pub fn selected_content_mut(&mut self) -> &mut text_editor::Content {
        match self.mode {
            BodyMode::Pretty => self.content.as_mut().unwrap_or(&mut self.raw),
            BodyMode::Raw => &mut self.raw,
        }
    }
}

#[derive(Debug, Default)]
pub enum ResponseState {
    #[default]
    Idle,
    Executing,
    Completed(CompletedResponse),
    Failed(Arc<anyhow::Error>),
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

    pub fn is_executing(&self) -> bool {
        matches!(self.state, ResponseState::Executing)
    }
}
