use crate::state::TabKey;
use core::http::collection::{FolderId, RequestId};
use core::http::environment::EnvironmentKey;
use core::http::CollectionKey;
use std::collections::BTreeMap;
use std::path::PathBuf;

use super::environment::{environment_keyvals, Env};
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
pub enum PopupNameAction {
    RenameCollection(CollectionKey),
    RenameFolder(CollectionKey, FolderId),
    RenameRequest(CollectionKey, RequestId),
    CreateFolder(CollectionKey, Option<FolderId>),
    NewRequest(CollectionKey, Option<FolderId>),
}

#[derive(Debug)]
pub struct PopupNameState {
    pub name: String,
    pub action: PopupNameAction,
}

#[derive(Debug)]
pub struct EnvironmentEditorState {
    pub col: CollectionKey,
    pub environments: BTreeMap<EnvironmentKey, Env>,
    pub deleted: Vec<EnvironmentKey>,
    pub selected_env: Option<EnvironmentKey>,
    pub env_name: String,
    pub add_env_mode: bool,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    EnvironmentEditor(EnvironmentEditorState),
    SaveRequest(SaveRequestState),
    PopupName(PopupNameState),
}

fn open_popup(state: &mut AppState, popup: Popup) {
    state.popup = Some(popup);
}

impl Popup {
    pub fn close(state: &mut AppState) {
        let Some(popup) = state.popup.take() else {
            return;
        };
        state.popup = match popup {
            Popup::EnvironmentEditor(mut data) if data.add_env_mode => {
                data.add_env_mode = false;
                Some(Popup::EnvironmentEditor(data))
            }
            _ => None,
        };
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

    pub fn popup_name(state: &mut AppState, name: String, action: PopupNameAction) {
        let popup = Self::PopupName(PopupNameState { name, action });
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
        let Some(envs) = state.collections.get_envs(col) else {
            return;
        };
        let popup = Self::EnvironmentEditor(EnvironmentEditorState {
            col,
            environments: environment_keyvals(envs),
            deleted: Vec::new(),
            selected_env: None,
            env_name: String::new(),
            add_env_mode: false,
        });
        open_popup(state, popup);
    }
}
