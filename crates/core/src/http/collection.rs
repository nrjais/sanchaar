use super::environment::{Environment, EnvironmentChain, EnvironmentKey};
use super::KeyValList;
use crate::http::environment::Environments;
use crate::new_id_type;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds};
use std::sync::Arc;
use std::time::Duration;

new_id_type! {
    pub struct RequestId;
    pub struct FolderId;
    pub struct CollectionKey;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub entries: Vec<Entry>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestRef {
    pub id: RequestId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entry {
    Item(RequestRef),
    Folder(Folder),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub content: String,
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub entries: Vec<Entry>,
    pub expanded: bool,
    pub environments: Environments,
    pub scripts: Vec<Script>,
    pub active_environment: Option<EnvironmentKey>,
    pub default_env: Option<EnvironmentKey>,
    #[serde_as(as = "Arc<_>")]
    pub headers: Arc<KeyValList>,
    #[serde_as(as = "Arc<_>")]
    pub variables: Arc<KeyValList>,
    #[serde_as(as = "Arc<_>")]
    pub dotenv: Arc<KeyValList>,
    pub disable_ssl: bool,
    #[serde_as(as = "DurationMilliSeconds")]
    pub timeout: Duration,
}

impl Collection {
    pub fn new(name: String, id: String) -> Self {
        Self {
            id,
            name,
            entries: vec![],
            expanded: false,
            environments: Environments::new(),
            scripts: Vec::new(),
            active_environment: None,
            default_env: None,
            headers: Default::default(),
            variables: Default::default(),
            dotenv: Default::default(),
            disable_ssl: false,
            timeout: Duration::from_secs(300),
        }
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut {
            stack: self.entries.iter_mut().collect::<Vec<_>>(),
        }
    }

    fn iter(&self) -> Iter {
        Iter {
            stack: self.entries.iter().collect::<Vec<_>>(),
        }
    }

    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn toggle_folder(&mut self, id: FolderId) {
        if let Some(folder) = self.folder_mut(id) {
            folder.expanded = !folder.expanded;
        }
    }

    pub fn folder_mut(&mut self, id: FolderId) -> Option<&mut Folder> {
        fn recurse<'a>(
            entries: &mut impl Iterator<Item = &'a mut Entry>,
            id: FolderId,
        ) -> Option<&'a mut Folder> {
            for entry in entries {
                if let Entry::Folder(folder) = entry {
                    if folder.id == id {
                        return Some(folder);
                    } else if let Some(folder) = recurse(&mut folder.entries.iter_mut(), id) {
                        return Some(folder);
                    }
                }
            }
            None
        }
        recurse(&mut self.entries.iter_mut(), id)
    }

    pub fn folder(&self, id: FolderId) -> Option<&Folder> {
        fn recurse<'a>(
            entries: impl Iterator<Item = &'a Entry>,
            id: FolderId,
        ) -> Option<&'a Folder> {
            for entry in entries {
                if let Entry::Folder(folder) = entry {
                    if folder.id == id {
                        return Some(folder);
                    } else if let Some(folder) = recurse(folder.entries.iter(), id) {
                        return Some(folder);
                    }
                }
            }
            None
        }
        recurse(self.entries.iter(), id)
    }

    pub fn get_active_environment(&self) -> Option<&Environment> {
        self.active_environment
            .and_then(|key| self.environments.get(key))
    }

    pub fn rename_request(&mut self, id: RequestId, name: &str) -> bool {
        for entry in self.iter_mut() {
            if let Entry::Item(item) = entry {
                if item.id == id {
                    item.name = name.to_string();
                    return true;
                }
            }
        }
        false
    }

    pub fn rename_folder(&mut self, id: FolderId, name: &str) -> bool {
        for entry in self.iter_mut() {
            if let Entry::Folder(item) = entry {
                if item.id == id {
                    item.name = name.to_string();
                    return true;
                }
            }
        }
        false
    }

    pub fn get_ref(&self, id: RequestId) -> Option<&RequestRef> {
        for entry in self.iter() {
            if let Entry::Item(item) = entry {
                if item.id == id {
                    return Some(item);
                }
            }
        }
        None
    }

    pub fn delete_folder(&mut self, folder_id: FolderId) -> bool {
        fn recurse(entries: &mut Vec<Entry>, id: FolderId) -> bool {
            let initial_len = entries.len();
            entries.retain(|e| !matches!(e, Entry::Folder(f) if f.id == id));

            if entries.len() != initial_len {
                return true;
            }

            for entry in entries.iter_mut() {
                if let Entry::Folder(folder) = entry {
                    if recurse(&mut folder.entries, id) {
                        return true;
                    }
                }
            }
            false
        }
        recurse(&mut self.entries, folder_id)
    }

    pub fn create_folder(&mut self, name: String, folder_id: Option<FolderId>) -> FolderId {
        let new_folder_id = FolderId::new();
        let new_entry = Entry::Folder(Folder {
            name,
            id: new_folder_id,
            entries: Vec::new(),
            expanded: true,
        });

        if let Some(parent_id) = folder_id {
            if let Some(folder) = self.folder_mut(parent_id) {
                folder.expanded = true;
                folder.entries.push(new_entry);
            }
        } else {
            self.entries.push(new_entry);
            self.expanded = true;
        }

        new_folder_id
    }

    pub fn create_request(&mut self, name: String, folder_id: Option<FolderId>) -> RequestId {
        let request_id = RequestId::new();
        let new_entry = Entry::Item(RequestRef {
            id: request_id,
            name,
        });

        if let Some(parent_id) = folder_id {
            if let Some(folder) = self.folder_mut(parent_id) {
                folder.entries.push(new_entry);
            }
        } else {
            self.entries.push(new_entry);
        }

        request_id
    }

    pub fn update_environment(&mut self, key: EnvironmentKey, env: Environment) {
        self.environments.update(key, env);
    }

    pub fn rename(&mut self, new: &str) {
        self.name = new.to_string();
    }

    pub fn delete_request(&mut self, req: RequestId) -> bool {
        fn recurse(entries: &mut Vec<Entry>, id: RequestId) -> bool {
            let initial_len = entries.len();
            entries.retain(|e| !matches!(e, Entry::Item(r) if r.id == id));

            if entries.len() != initial_len {
                return true;
            }

            for entry in entries.iter_mut() {
                if let Entry::Folder(folder) = entry {
                    if recurse(&mut folder.entries, id) {
                        return true;
                    }
                }
            }
            false
        }
        recurse(&mut self.entries, req)
    }

    pub fn delete_environment(&mut self, key: EnvironmentKey) -> Option<Environment> {
        self.environments.remove(key)
    }

    pub fn create_script(&mut self, name: String, content: String) -> String {
        let script = Script {
            name: name.clone(),
            content,
        };
        self.scripts.push(script);
        name
    }

    pub fn get_script(&self, name: &str) -> Option<&Script> {
        self.scripts.iter().find(|script| script.name == name)
    }

    pub fn update_active_env_by_name(&mut self, name: &str) {
        let env = self.environments.find_by_name(name);

        if let Some(env) = env {
            self.active_environment = Some(env);
        }
    }

    pub fn set_default_env(&mut self, env: Option<EnvironmentKey>) {
        self.default_env = env;
        if self.active_environment.is_none() {
            self.active_environment = env;
        }
    }

    pub fn env_chain(&self) -> EnvironmentChain {
        let env = self
            .get_active_environment()
            .map(|e| e.vars())
            .unwrap_or_default();

        EnvironmentChain::from_iter(
            Arc::clone(&self.dotenv),
            [env, Arc::clone(&self.variables)].into_iter(),
        )
    }

    pub fn collection_env_chain(&self) -> EnvironmentChain {
        let env = self
            .get_active_environment()
            .map(|e| e.vars())
            .unwrap_or_default();

        EnvironmentChain::from_iter(Arc::clone(&self.dotenv), [env].into_iter())
    }

    pub fn dotenv_env_chain(&self) -> EnvironmentChain {
        EnvironmentChain::from_iter(Arc::clone(&self.dotenv), [].into_iter())
    }
}

struct IterMut<'a> {
    stack: Vec<&'a mut Entry>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Entry;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.stack.pop()?;

            match node {
                Entry::Folder(Folder { entries, .. }) => {
                    self.stack.extend(entries.iter_mut());
                }
                Entry::Item(_) => return Some(node),
            };
        }
    }
}

struct Iter<'a> {
    stack: Vec<&'a Entry>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        return match node {
            Entry::Folder(Folder { entries, .. }) => {
                self.stack.extend(entries.iter());
                Some(node)
            }
            Entry::Item(_) => Some(node),
        };
    }
}
