use core::http::environment::{Environment, EnvironmentKey, Environments};
use std::collections::BTreeMap;

use components::KeyValList;

use super::utils::{from_core_kv_list, to_core_kv_list};

#[derive(Debug, Clone)]
pub struct Env {
    pub name: String,
    pub variables: KeyValList,
}

impl From<&Environment> for Env {
    fn from(env: &Environment) -> Self {
        Self {
            name: env.name.clone(),
            variables: from_core_kv_list(env.variables.clone(), false),
        }
    }
}

impl From<&Env> for Environment {
    fn from(value: &Env) -> Self {
        Self {
            name: value.name.trim().to_owned(),
            variables: to_core_kv_list(&value.variables),
        }
    }
}

pub fn environment_keyvals(envs: &Environments) -> BTreeMap<EnvironmentKey, Env> {
    envs.entries()
        .map(|(key, env)| (*key, Env::from(env)))
        .collect()
}
