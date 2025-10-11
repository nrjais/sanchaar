use lib::http::{CollectionKey, collection::Collection};
use std::time::Duration;

use crate::components::KeyValList;

use crate::state::environment::EnvironmentsEditor;
use crate::state::{environment::environment_keyvals, utils::from_core_kv_list};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CollectionTabId {
    #[default]
    Settings,
    Environments,
}

#[derive(Debug)]
pub struct CollectionTab {
    pub name: String,
    pub default_env: Option<String>,
    pub collection_key: CollectionKey,
    pub tab: CollectionTabId,
    pub env_editor: EnvironmentsEditor,
    pub headers: KeyValList,
    pub disable_ssl: bool,
    pub timeout: Duration,
    pub timeout_str: String,
    pub edited: bool,
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
            headers: from_core_kv_list(&col.headers, false),
            env_editor: environment_keyvals(&col.environments),
            disable_ssl: col.disable_ssl,
            timeout: col.timeout,
            timeout_str: col.timeout.as_millis().to_string(),
            edited: false,
        }
    }
}
