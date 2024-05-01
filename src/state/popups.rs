use crate::state::TabKey;
use core::http::collection::FolderId;
use core::http::environment::EnvironmentKey;
use core::http::CollectionKey;
use std::path::PathBuf;

use super::AppState;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CreateFolderState {
    pub name: String,
    pub col: CollectionKey,
    pub folder_id: Option<FolderId>,
}

#[derive(Debug)]
pub struct EnvironmentEditorState {
    pub col: CollectionKey,
    pub selected_env: Option<EnvironmentKey>,
    pub env_name: String,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(EnvironmentEditorState),
    SaveRequest(SaveRequestState),
    CreateFolder(CreateFolderState),
}

fn open_popup(state: &mut AppState, popup: Popup) {
    state.popup = Some(popup);
}

impl Popup {
    pub fn close(state: &mut AppState) {
        state.popup = None;
    }

    pub fn save_request(state: &mut AppState, tab: TabKey) {
        let popup = Self::SaveRequest(SaveRequestState {
            tab,
            name: String::new(),
            col: None,
            folder_id: None,
        });
        open_popup(state, popup);
    }

    pub fn create_folder(state: &mut AppState, col: CollectionKey, folder_id: Option<FolderId>) {
        let popup = Self::CreateFolder(CreateFolderState {
            name: String::new(),
            col,
            folder_id,
        });
        open_popup(state, popup);
    }

    pub fn create_collection(state: &mut AppState) {
        let popup = Self::CreateCollection(CreateCollectionState {
            name: String::new(),
            path: None,
        });
        open_popup(state, popup);
    }

    pub fn environment_editor(state: &mut AppState, col: CollectionKey) {
        let popup = Self::EnvironmentEditor(EnvironmentEditorState {
            col,
            selected_env: None,
            env_name: String::new(),
        });
        open_popup(state, popup);
    }
}
