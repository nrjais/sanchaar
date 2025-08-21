use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::new_id_type;
use parsers::{Token, parse_template};

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

#[derive(Debug, Clone, Default)]
pub struct EnvironmentChain {
    dotenv: Arc<KeyValList>,
    vars: Vec<Arc<KeyValList>>,
}

impl EnvironmentChain {
    pub fn new() -> Self {
        Self {
            dotenv: Default::default(),
            vars: Vec::new(),
        }
    }

    pub fn from_iter<I>(dotenv: Arc<KeyValList>, iter: I) -> Self
    where
        I: IntoIterator<Item = Arc<KeyValList>>,
    {
        Self {
            dotenv,
            vars: iter.into_iter().collect(),
        }
    }

    fn get_named(name: &str, list: &KeyValList) -> Option<String> {
        list.iter()
            .rev()
            .find(|kv| kv.name == name)
            .map(|kv| kv.value.to_owned())
    }

    pub fn all_var_set(&self) -> Arc<HashSet<String>> {
        let mut set = HashSet::new();
        for vars in self.vars.iter().chain([&self.dotenv]) {
            for kv in vars.iter() {
                set.insert(kv.name.clone());
            }
        }
        Arc::new(set)
    }

    fn replace_dotenv(&self, source: &str) -> String {
        let mut buffer = String::new();
        for span in parse_template(source) {
            match span.token {
                Token::Text(text) => buffer.push_str(&text),
                Token::Variable(var) => {
                    let value = Self::get_named(&var, &self.dotenv).unwrap_or(var);
                    buffer.push_str(value.as_str());
                }
                Token::Escaped(text) => {
                    buffer.push_str(&text);
                }
            }
        }
        buffer
    }

    pub fn get(&self, name: &str) -> Option<String> {
        let name = name.trim_ascii();
        Self::get_named(name, &self.dotenv).or_else(|| {
            self.vars
                .iter()
                .find_map(|vars| Self::get_named(name, vars))
                .to_owned()
                .map(|s| self.replace_dotenv(&s))
        })
    }
}
