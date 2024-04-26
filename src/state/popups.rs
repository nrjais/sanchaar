use std::path::PathBuf;
use core::collection::CollectionKey;

#[derive(Debug, Default)]
pub struct CreateCollectionState {
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(CollectionKey),
}
