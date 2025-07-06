use core::persistence::history::HistoryEntrySummary;
use std::time::Instant;

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
    pub search_query: String,
    pub is_searching: bool,
    pub last_search_input: Option<Instant>,
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
            search_query: String::new(),
            is_searching: false,
            last_search_input: None,
        }
    }

    pub fn set_entries(&mut self, entries: Vec<HistoryEntrySummary>) {
        self.entries = entries;
        self.error = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
        self.last_search_input = Some(Instant::now());
    }

    pub fn set_searching(&mut self, searching: bool) {
        self.is_searching = searching;
    }

    pub fn should_trigger_search(&self) -> bool {
        if let Some(last_input) = self.last_search_input {
            last_input.elapsed().as_millis() >= 300
        } else {
            false
        }
    }

    pub fn clear_search_timer(&mut self) {
        self.last_search_input = None;
    }

    pub fn clear_search_query(&mut self) {
        self.search_query.clear();
        self.last_search_input = None;
        self.is_searching = false;
    }
}
