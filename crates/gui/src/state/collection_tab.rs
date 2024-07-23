use core::http::{collection::Collection, environment::EnvironmentKey, CollectionKey};
use std::collections::BTreeMap;

use super::environment::{environment_keyvals, Env};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CollectionTabId {
    #[default]
    Settings,
    Environments,
}

#[derive(Debug)]
pub struct EnvironmentEditorState {
    pub col: CollectionKey,
    pub environments: BTreeMap<EnvironmentKey, Env>,
    pub deleted: Vec<EnvironmentKey>,
    pub selected_env: Option<EnvironmentKey>,
    pub edited: bool,
}

#[derive(Debug)]
pub struct CollectionTab {
    pub name: String,
    pub tab: CollectionTabId,
    pub env_editor: EnvironmentEditorState,
}

impl CollectionTab {
    pub fn env_tab(key: CollectionKey, col: &Collection) -> Self {
        CollectionTab {
            name: col.name.clone(),
            tab: CollectionTabId::Environments,
            env_editor: EnvironmentEditorState {
                col: key,
                environments: environment_keyvals(&col.environments),
                deleted: Vec::new(),
                selected_env: None,
                edited: false,
            },
        }
    }
}
