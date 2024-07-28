use std::{collections::HashMap, sync::Arc};

use crate::new_id_type;

use super::KeyValList;

new_id_type! {
    pub struct EnvironmentKey;
}

#[derive(Debug, Clone)]
pub struct Environments {
    envs: HashMap<EnvironmentKey, Environment>,
}

impl Default for Environments {
    fn default() -> Self {
        Self::new()
    }
}

impl Environments {
    pub fn new() -> Self {
        Self {
            envs: HashMap::new(),
        }
    }

    pub fn get(&self, id: EnvironmentKey) -> Option<&Environment> {
        self.envs.get(&id)
    }

    pub fn get_mut(&mut self, id: &EnvironmentKey) -> Option<&mut Environment> {
        self.envs.get_mut(id)
    }

    pub fn insert(&mut self, env: Environment) -> EnvironmentKey {
        let id = EnvironmentKey::new();
        self.envs.insert(id, env);
        id
    }

    pub fn update(&mut self, id: EnvironmentKey, env: Environment) {
        self.envs.insert(id, env);
    }

    pub fn entries(&self) -> impl Iterator<Item = (&EnvironmentKey, &Environment)> {
        self.envs.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.envs.is_empty()
    }

    pub fn create(&mut self, name: String) -> EnvironmentKey {
        let env = Environment::new(name);
        self.insert(env)
    }

    pub fn find_by_name(&self, name: &str) -> Option<EnvironmentKey> {
        self.envs
            .iter()
            .find(|(_, env)| env.name == name)
            .map(|(id, _)| *id)
    }

    pub fn remove(&mut self, key: EnvironmentKey) -> Option<Environment> {
        self.envs.remove(&key)
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub variables: Arc<KeyValList>,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: Default::default(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables
            .iter()
            .rev()
            .find(|kv| kv.name == name)
            .map(|kv| kv.value.as_str())
    }

    pub fn vars(&self) -> Arc<KeyValList> {
        Arc::clone(&self.variables)
    }
}

#[derive(Debug, Clone)]
struct WithPath {
    path: String,
    vars: Arc<KeyValList>,
}

impl WithPath {
    fn new(path: String, vars: Arc<KeyValList>) -> Self {
        Self { path, vars }
    }

    fn get_named(&self, name: &str) -> Option<&str> {
        self.vars
            .iter()
            .rev()
            .find(|kv| kv.name == name)
            .map(|kv| kv.value.as_str())
    }

    fn get(&self, name: &str) -> Option<&str> {
        let (path, name) = name.split_once('.').unwrap_or(("", name));
        if path == self.path {
            self.get_named(name)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvironmentChain {
    vars: Vec<WithPath>,
}

impl EnvironmentChain {
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Arc<KeyValList>)>,
    {
        Self {
            vars: iter
                .into_iter()
                .map(|(path, vars)| WithPath::new(path, vars))
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.vars.iter().find_map(|env| env.get(name))
    }
}
