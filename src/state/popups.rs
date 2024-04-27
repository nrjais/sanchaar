use crate::state::TabKey;
use core::http::collection::FolderId;
use core::http::CollectionKey;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct CreateCollectionState {
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct SaveRequestState {
    pub tab: TabKey,
    pub name: String,
    pub col: Option<CollectionKey>,
    pub folder_id: Option<FolderId>,
}

impl SaveRequestState {
    pub fn new(tab: TabKey) -> Self {
        Self {
            tab,
            name: String::new(),
            col: None,
            folder_id: None,
        }
    }
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(CollectionKey),
    SaveRequest(SaveRequestState),
}
