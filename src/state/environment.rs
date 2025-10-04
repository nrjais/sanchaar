use core::http::{Environment, EnvironmentKey, environment::Environments};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::components::editor;

#[derive(Debug, Default)]
pub struct EnvVariable {
    pub name: editor::Content,
    pub values: HashMap<EnvironmentKey, editor::Content>,
}

#[derive(Debug)]
pub struct EnvironmentsEditor {
    pub variables: Vec<EnvVariable>,
    pub environments: HashMap<EnvironmentKey, Environment>,
    pub edited: bool,
}

impl EnvironmentsEditor {
    pub fn add_env(&mut self, env: Environment) {
        self.environments.insert(EnvironmentKey::new(), env);
        self.edited = true;
    }

    pub fn remove_env(&mut self, env_key: EnvironmentKey) -> Option<Environment> {
        self.edited = true;
        for variable in self.variables.iter_mut() {
            variable.values.remove(&env_key);
        }
        self.environments.remove(&env_key)
    }

    pub fn add_variable(&mut self) {
        let name = editor::Content::new();
        let values = self
            .environments
            .keys()
            .map(|key| (*key, editor::Content::new()))
            .collect();

        self.variables.push(EnvVariable { name, values });
        self.edited = true;
    }

    pub(crate) fn create_env(&mut self, name: String) {
        let env = Environment::new(name);
        let env_key = EnvironmentKey::new();
        self.environments.insert(env_key, env);
        self.edited = true;
        for variable in self.variables.iter_mut() {
            variable.values.insert(env_key, editor::Content::new());
        }
    }

    pub fn get_envs_for_save(&mut self) -> HashMap<EnvironmentKey, Environment> {
        self.edited = false;
        let mut envs = HashMap::new();
        for variable in self.variables.iter() {
            for (env_key, content) in variable.values.iter() {
                let env = envs.entry(*env_key).or_insert_with(HashMap::new);
                env.insert(variable.name.text(), content.text());
            }
        }

        let envname = |key: &EnvironmentKey| self.environments.get(key).map(|env| env.name.clone());

        envs.into_iter()
            .filter_map(|(key, env)| {
                Some((
                    key,
                    Environment {
                        name: envname(&key)?,
                        variables: Arc::new(env),
                    },
                ))
            })
            .collect()
    }
}

pub fn environment_keyvals(envs: &Environments) -> EnvironmentsEditor {
    let environments: HashMap<EnvironmentKey, Environment> = envs
        .entries()
        .map(|(key, env)| (*key, env.clone()))
        .collect();

    let mut variables = envs
        .entries()
        .flat_map(|(_, env)| env.variables.keys().cloned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    variables.sort();

    let variables = variables
        .into_iter()
        .map(|name| EnvVariable {
            name: editor::Content::with_text(name.as_str()),
            values: environments
                .iter()
                .map(|(key, env)| (*key, env.variables.get(&name).cloned().unwrap_or_default()))
                .map(|(key, value)| (key, editor::Content::with_text(value.as_str())))
                .collect(),
        })
        .collect();

    EnvironmentsEditor {
        variables,
        environments,
        edited: false,
    }
}
