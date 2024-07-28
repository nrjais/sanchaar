use core::http::environment::{Environment, EnvironmentKey, Environments};
use std::{collections::BTreeMap, sync::Arc};

use components::KeyValList;

use super::utils::{from_core_kv_list, to_core_kv_list};

#[derive(Debug)]
pub struct Env {
    pub name: String,
    pub variables: KeyValList,
}
impl Env {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: KeyValList::new(),
        }
    }
}

impl From<&Environment> for Env {
    fn from(env: &Environment) -> Self {
        Self {
            name: env.name.clone(),
            variables: from_core_kv_list(&env.variables, false),
        }
    }
}

impl From<&Env> for Environment {
    fn from(value: &Env) -> Self {
        Self {
            name: value.name.trim().to_owned(),
            variables: Arc::new(to_core_kv_list(&value.variables)),
        }
    }
}

pub fn environment_keyvals(envs: &Environments) -> BTreeMap<EnvironmentKey, Env> {
    envs.entries()
        .map(|(key, env)| (*key, Env::from(env)))
        .collect()
}
