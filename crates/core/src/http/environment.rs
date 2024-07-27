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
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: Default::default(),
            parent: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables
            .iter()
            .rev()
            .find(|kv| kv.name == name)
            .map(|kv| kv.value.as_str())
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(name)))
    }

    pub fn inherit(&self, parent: Arc<KeyValList>) -> Self {
        Environment {
            name: self.name.clone(),
            variables: Arc::clone(&self.variables),
            parent: Some(Box::new(Environment {
                name: self.name.clone(),
                variables: parent,
                parent: None,
            })),
        }
    }
}
