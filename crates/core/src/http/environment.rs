use slotmap::SlotMap;

use super::request::KeyValList;

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

    pub fn is_empty(&self) -> bool {
        self.envs.is_empty()
    }

    pub(crate) fn create(&mut self, name: String) -> EnvironmentKey {
        let env = Environment::new(name);
        self.insert(env)
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub variables: KeyValList,
}

impl Environment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            variables: KeyValList::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables
            .iter()
            .find(|kv| kv.name == name)
            .map(|kv| kv.value.as_str())
    }
}
