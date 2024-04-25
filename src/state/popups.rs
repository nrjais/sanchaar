use crate::state::TabKey;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct CreateCollectionState {
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(TabKey),
}
