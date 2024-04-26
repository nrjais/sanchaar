use slotmap::SlotMap;
use std::collections::HashMap;

slotmap::new_key_type! {
    pub struct EnvironmentKey;
}

#[derive(Debug, Clone)]
pub struct Environments {
    envs: SlotMap<EnvironmentKey, Environment>,
}

impl Environments {
    pub fn new() -> Self {
        Self {
            envs: SlotMap::with_key(),
        }
    }

    pub fn get(&self, id: EnvironmentKey) -> Option<&Environment> {
        self.envs.get(id)
    }

    pub fn get_mut(&mut self, id: &EnvironmentKey) -> Option<&mut Environment> {
        self.envs.get_mut(*id)
    }

    pub fn insert(&mut self, env: Environment) -> EnvironmentKey {
        self.envs.insert(env)
    }

    pub fn entries(&self) -> impl Iterator<Item = (EnvironmentKey, &Environment)> {
        self.envs.iter()
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub variables: HashMap<String, String>,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    pub fn set(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
}
