use lib::persistence::history::HistoryEntrySummary;
use std::time::Instant;

use crate::components::{
    LineEditorMsg,
    editor::{self, ContentAction},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryTabId {
    List,
}

#[derive(Debug)]
pub struct HistoryTab {
    pub name: String,
    pub tab: HistoryTabId,
    pub entries: Vec<HistoryEntrySummary>,
    pub error: Option<String>,
    pub search_query: editor::Content,
    pub is_searching: bool,
    pub last_search_input: Option<Instant>,
    pub search_query_text: String,
}

impl Default for HistoryTab {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryTab {
    pub fn new() -> Self {
        Self {
            name: "History".to_string(),
            tab: HistoryTabId::List,
            entries: Vec::new(),
            error: None,
            search_query: editor::Content::new(),
            is_searching: false,
            last_search_input: None,
            search_query_text: String::new(),
        }
    }

    pub fn set_entries(&mut self, entries: Vec<HistoryEntrySummary>) {
        self.entries = entries;
        self.error = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn set_search_query(&mut self, msg: LineEditorMsg) {
        msg.update(&mut self.search_query);
        self.last_search_input = Some(Instant::now());
        self.search_query_text = self.search_query.text().trim().to_string();
    }

    pub fn set_searching(&mut self, searching: bool) {
        self.is_searching = searching;
    }

    pub fn should_trigger_search(&self) -> bool {
        if let Some(last_input) = self.last_search_input {
            last_input.elapsed().as_millis() >= 100
        } else {
            false
        }
    }

    pub fn clear_search_timer(&mut self) {
        self.last_search_input = None;
    }

    pub fn clear_search_query(&mut self) {
        self.search_query
            .perform(ContentAction::Replace("".to_string()));
        self.last_search_input = None;
        self.is_searching = false;
    }
}
