use core::http::environment::{Environment, EnvironmentKey, Environments};
use std::collections::HashMap;

use components::{KeyValList, KeyValue};

#[derive(Debug, Clone)]
pub struct Env {
    pub name: String,
    pub variables: KeyValList,
}

fn to_keyval_list(env: &Environment) -> KeyValList {
    KeyValList::from(
        env.variables
            .iter()
            .map(|(name, value)| KeyValue {
                disabled: false,
                name: name.clone(),
                value: value.clone(),
            })
            .collect(),
        false,
    )
}

impl From<&Environment> for Env {
    fn from(env: &Environment) -> Self {
        Self {
            name: env.name.clone(),
            variables: to_keyval_list(env),
        }
    }
}

pub fn environment_keyvals(envs: &Environments) -> HashMap<EnvironmentKey, Env> {
    envs.entries()
        .map(|(key, env)| (key, Env::from(env)))
        .collect()
}
