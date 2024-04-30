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

#[derive(Debug, Default)]
pub struct CreateFolderState {
    pub name: String,
    pub col: CollectionKey,
    pub folder_id: Option<FolderId>,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(CollectionKey),
    SaveRequest(SaveRequestState),
    CreateFolder(CreateFolderState),
}

impl Popup {
    pub fn save_request(tab: TabKey) -> Self {
        Self::SaveRequest(SaveRequestState {
            tab,
            name: String::new(),
            col: None,
            folder_id: None,
        })
    }

    pub fn create_folder(col: CollectionKey, folder_id: Option<FolderId>) -> Self {
        Self::CreateFolder(CreateFolderState {
            name: String::new(),
            col,
            folder_id,
        })
    }
}
