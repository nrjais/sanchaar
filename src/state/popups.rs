use iced_auto_updater_plugin::ReleaseInfo;

use crate::state::TabKey;
use lib::http::CollectionKey;
use lib::http::collection::{FolderId, RequestId};
use lib::http::environment::EnvironmentKey;
use std::path::PathBuf;

use super::CommonState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionCreationMode {
    CreateNew,
    ImportPostman,
}

#[derive(Debug)]
pub struct CreateCollectionState {
    pub mode: CollectionCreationMode,
    pub name: String,
    pub path: Option<PathBuf>,
    // For import mode
    pub import_file_path: Option<PathBuf>,
    pub import_target_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct SaveRequestState {
    pub tab: TabKey,
    pub name: String,
    pub col: Option<CollectionKey>,
    pub folder_id: Option<FolderId>,
}

#[derive(Debug, Clone)]
pub enum PopupNameAction {
    RenameCollection(CollectionKey),
    RenameFolder(CollectionKey, FolderId),
    RenameRequest(CollectionKey, RequestId),
    CreateFolder(CollectionKey, Option<FolderId>),
    NewRequest(CollectionKey, Option<FolderId>),
    NewScript(CollectionKey),
    RenameScript(CollectionKey, String),
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
pub struct UpdateConfirmationState(pub ReleaseInfo);

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
    SaveRequest(SaveRequestState),
    PopupName(PopupNameState),
    AppSettings(AppSettingsState),
    UpdateConfirmation(UpdateConfirmationState),
}

impl Popup {
    pub fn done(&self) -> &'static str {
        match self {
            Popup::CreateCollection(state) => match state.mode {
                CollectionCreationMode::CreateNew => "Create",
                CollectionCreationMode::ImportPostman => "Import",
            },
            Popup::SaveRequest(_) => "Save",
            Popup::PopupName(_) => "Ok",
            Popup::AppSettings(_) => "Done",
            Popup::UpdateConfirmation(_) => "Update",
        }
    }
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
            mode: CollectionCreationMode::CreateNew,
            name: String::new(),
            path: None,
            import_file_path: None,
            import_target_path: None,
        });
        open_popup(state, popup);
    }

    pub fn app_settings(state: &mut CommonState) {
        let popup = Self::AppSettings(AppSettingsState {
            active_tab: AppSettingTabs::General,
        });
        open_popup(state, popup);
    }

    pub fn update_confirmation(state: &mut CommonState, release: ReleaseInfo) {
        let popup = Self::UpdateConfirmation(UpdateConfirmationState(release));
        open_popup(state, popup);
    }
}
