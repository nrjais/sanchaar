use core::persistence::history::HistoryEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryTabId {
    List,
}

#[derive(Debug)]
pub struct HistoryTab {
    pub name: String,
    pub tab: HistoryTabId,
    pub entries: Vec<HistoryEntry>,
    pub error: Option<String>,
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
        }
    }

    pub fn set_entries(&mut self, entries: Vec<HistoryEntry>) {
        self.entries = entries;
        self.error = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }
}
