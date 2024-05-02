use core::http::environment::{Environment, EnvironmentKey, Environments};
use std::collections::HashMap;

use components::KeyValList;

use super::utils::from_core_kv_list;

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

pub fn environment_keyvals(envs: &Environments) -> HashMap<EnvironmentKey, Env> {
    envs.entries()
        .map(|(key, env)| (key, Env::from(env)))
        .collect()
}
