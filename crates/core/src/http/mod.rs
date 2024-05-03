use std::path::PathBuf;

use slotmap::SlotMap;

use crate::http::collection::{Collection, RequestId, RequestRef};
use crate::http::environment::Environments;

use self::environment::{Environment, EnvironmentKey};

pub mod collection;
pub mod environment;
pub mod request;

slotmap::new_key_type! {
    pub struct CollectionKey;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyValue {
    pub disabled: bool,
    pub name: String,
    pub value: String,
}

pub type KeyValList = Vec<KeyValue>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectionRequest(pub CollectionKey, pub RequestId);

#[derive(Debug, Default)]
pub struct Collections {
    pub entries: SlotMap<CollectionKey, Collection>,
    pub dirty: bool,
}

impl Collections {
    pub fn with_collection_mut<F, R>(&mut self, key: CollectionKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut Collection) -> R,
    {
        self.dirty();
        self.entries.get_mut(key).map(f)
    }

    pub fn with_collection<F, R>(&self, key: CollectionKey, f: F) -> Option<R>
    where
        F: FnOnce(&Collection) -> R,
    {
        self.entries.get(key).map(f)
    }

    pub fn get_ref(&self, cr: CollectionRequest) -> Option<&RequestRef> {
        self.entries.get(cr.0).and_then(|c| c.get_ref(cr.1))
    }

    pub fn get(&self, key: CollectionKey) -> Option<&Collection> {
        self.entries.get(key)
    }
    pub fn get_mut(&mut self, key: CollectionKey) -> Option<&mut Collection> {
        self.dirty();
        self.entries.get_mut(key)
    }

    pub fn insert_all(&mut self, collections: Vec<Collection>) {
        self.dirty();
        for collection in collections {
            self.entries.insert(collection);
        }
    }

    pub fn insert(&mut self, collection: Collection) {
        self.dirty();
        self.entries.insert(collection);
    }

    pub fn get_envs(&self, key: CollectionKey) -> Option<&Environments> {
        Some(&self.entries.get(key)?.environments)
    }

    pub fn get_env(&self, key: CollectionKey, env: EnvironmentKey) -> Option<Environment> {
        let envs = self.get_envs(key)?;
        envs.get(env).cloned()
    }

    pub fn rename_request(
        &mut self,
        req: CollectionRequest,
        new: String,
    ) -> Option<(PathBuf, PathBuf)> {
        self.entries.get_mut(req.0)?.rename_request(req.1, &new)
    }

    pub fn create_collection(&mut self, name: String, path: PathBuf) -> &Collection {
        let children = Vec::new();
        let path = path.join(&name);
        let collection = Collection::new(name, children, path, Environments::new());

        self.dirty();
        let key = self.entries.insert(collection);
        self.entries
            .get(key)
            .expect("Inserted collection not found")
    }

    fn dirty(&mut self) {
        self.dirty = true;
    }

    pub fn get_collections_for_save(&mut self) -> Vec<Collection> {
        self.dirty = false;
        self.entries.values().cloned().collect()
    }

    pub fn delete_folder(
        &mut self,
        col: CollectionKey,
        folder_id: collection::FolderId,
    ) -> Option<PathBuf> {
        self.with_collection_mut(col, |collection| collection.delete_folder(folder_id))
            .flatten()
    }

    pub fn create_folder_in(
        &mut self,
        name: String,
        col: CollectionKey,
        folder_id: Option<collection::FolderId>,
    ) -> Option<PathBuf> {
        self.entries
            .get_mut(col)
            .map(|collection| collection.create_folder(name, folder_id))
            .flatten()
    }

    pub fn remove(&mut self, col: CollectionKey) {
        self.dirty();
        self.entries.remove(col);
    }

    pub fn create_env(&mut self, col: CollectionKey, name: String) -> Option<EnvironmentKey> {
        let collection = self.entries.get_mut(col)?;

        Some(collection.environments.create(name))
    }

    pub fn find_env_by_name(&self, col: CollectionKey, name: &str) -> Option<EnvironmentKey> {
        self.entries.get(col)?.environments.find_by_name(name)
    }
}
