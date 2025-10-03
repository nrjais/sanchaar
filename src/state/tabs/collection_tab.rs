use core::http::VarMap;
use core::http::{CollectionKey, collection::Collection};
use std::collections::HashMap;
use std::time::Duration;

use crate::components::{KeyValList, editor};

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
    pub variables: HashMap<String, editor::Content>,
    pub disable_ssl: bool,
    pub timeout: Duration,
    pub timeout_str: String,
    pub edited: bool,
}

fn var_map_editor(vars: &VarMap) -> HashMap<String, editor::Content> {
    vars.iter()
        .map(|(name, value)| (name.clone(), editor::Content::with_text(value)))
        .collect()
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
            variables: var_map_editor(&col.variables),
            env_editor: environment_keyvals(&col.environments),
            disable_ssl: col.disable_ssl,
            timeout: col.timeout,
            timeout_str: col.timeout.as_millis().to_string(),
            edited: false,
        }
    }
}
