use core::client;
use std::sync::Arc;

use components::editor::{self, Content};
use jsonpath_rust::JsonPath;
use serde_json::Value;

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
    pub content: Option<editor::Content>,
    pub raw: editor::Content,
    pub mode: BodyMode,
    pub filtered_content: Option<editor::Content>,
    pub json_path_filter: String,
    pub value: Option<Value>,
}

impl CompletedResponse {
    pub fn selected_content(&self) -> &editor::Content {
        if let Some(filtered_content) = &self.filtered_content {
            return filtered_content;
        }

        match self.mode {
            BodyMode::Pretty => self.content.as_ref().unwrap_or(&self.raw),
            BodyMode::Raw => &self.raw,
        }
    }
    pub fn selected_content_mut(&mut self) -> &mut editor::Content {
        if let Some(filtered_content) = &mut self.filtered_content {
            return filtered_content;
        }

        match self.mode {
            BodyMode::Pretty => self.content.as_mut().unwrap_or(&mut self.raw),
            BodyMode::Raw => &mut self.raw,
        }
    }

    pub fn apply_json_path_filter(&mut self) -> Option<()> {
        self.filtered_content = None;
        if self.json_path_filter.trim().is_empty() {
            return None;
        }

        if let Some(json) = &self.value {
            let path = JsonPath::try_from(self.json_path_filter.as_str()).ok()?;
            let filtered = path.find(json);
            let filtered_json = serde_json::to_string_pretty(&filtered).unwrap_or_default();
            self.filtered_content = Some(editor::Content::with_text(&filtered_json));
        }

        Some(())
    }

    pub fn new(res: client::Response) -> Self {
        let (raw, pretty, value) = pretty_body(&res.body.data);
        Self {
            result: res,
            content: pretty.map(|p| Content::with_text(p.as_str())),
            raw: Content::with_text(raw.as_str()),
            value,
            mode: BodyMode::Pretty,
            filtered_content: None,
            json_path_filter: String::new(),
        }
    }
}

fn pretty_body(body: &[u8]) -> (String, Option<String>, Option<Value>) {
    let raw = String::from_utf8_lossy(body).to_string();

    let value = serde_json::from_slice::<Value>(body).ok();
    let json = value
        .as_ref()
        .and_then(|v| serde_json::to_string_pretty(&v).ok());

    (raw, json, value)
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
