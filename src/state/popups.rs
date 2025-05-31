use crate::state::TabKey;
use core::http::collection::{FolderId, RequestId};
use core::http::environment::EnvironmentKey;
use core::http::CollectionKey;
use std::path::PathBuf;

use super::CommonState;

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
    NewScript(CollectionKey),
    CreateEnvironment(TabKey),
    RenameEnvironment(TabKey, EnvironmentKey),
}

#[derive(Debug)]
pub struct PopupNameState {
    pub name: String,
    pub action: PopupNameAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppSettingTabs {
    General,
}

#[derive(Debug)]
pub struct AppSettingsState {
    pub active_tab: AppSettingTabs,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    SaveRequest(SaveRequestState),
    PopupName(PopupNameState),
    AppSettings(AppSettingsState),
}

fn open_popup(state: &mut CommonState, popup: Popup) {
    state.popup = Some(popup);
}

impl Popup {
    pub fn close(state: &mut CommonState) {
        state.popup.take();
    }

    pub fn save_request(state: &mut CommonState, tab: TabKey) {
        let popup = Self::SaveRequest(SaveRequestState {
            tab,
            name: String::new(),
            col: None,
            folder_id: None,
        });
        open_popup(state, popup);
    }

    pub fn popup_name(state: &mut CommonState, name: String, action: PopupNameAction) {
        let popup = Self::PopupName(PopupNameState { name, action });
        open_popup(state, popup);
    }

    pub fn create_collection(state: &mut CommonState) {
        let popup = Self::CreateCollection(CreateCollectionState {
            name: String::new(),
            path: None,
        });
        open_popup(state, popup);
    }

    pub fn app_settings(state: &mut CommonState) {
        let popup = Self::AppSettings(AppSettingsState {
            active_tab: AppSettingTabs::General,
        });
        open_popup(state, popup);
    }
}
