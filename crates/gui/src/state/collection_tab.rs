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
pub struct EnvironmentEditor {
    pub environments: BTreeMap<EnvironmentKey, Env>,
    pub deleted: Vec<EnvironmentKey>,
    pub selected_env: Option<EnvironmentKey>,
    pub edited: bool,
}

#[derive(Debug)]
pub struct CollectionTab {
    pub name: String,
    pub default_env: Option<String>,
    pub collection_key: CollectionKey,
    pub tab: CollectionTabId,
    pub env_editor: EnvironmentEditor,
}

impl CollectionTab {
    pub fn new(key: CollectionKey, col: &Collection) -> Self {
        let default_env = col
            .default_env
            .as_ref()
            .and_then(|env| col.environments.get(*env))
            .map(|env| env.name.clone());

        CollectionTab {
            name: col.name.clone(),
            tab: CollectionTabId::Settings,
            default_env,
            collection_key: key,
            env_editor: EnvironmentEditor {
                environments: environment_keyvals(&col.environments),
                deleted: Vec::new(),
                selected_env: col.active_environment,
                edited: false,
            },
        }
    }

    pub fn add_env(&mut self, env: Env) {
        self.env_editor
            .environments
            .insert(EnvironmentKey::new(), env);
        self.env_editor.edited = true;
    }

    pub fn remove_env(&mut self, env_key: EnvironmentKey) -> Option<Env> {
        self.env_editor.edited = true;
        if self.env_editor.selected_env == Some(env_key) {
            self.env_editor.selected_env = None;
        }
        self.env_editor.deleted.push(env_key);
        self.env_editor.environments.remove(&env_key)
    }
}
