use crate::state::TabKey;
use core::http::collection::RequestId;
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
    pub folder_id: Option<RequestId>,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(CollectionKey),
    SaveRequest(SaveRequestState),
}
