use std::path::PathBuf;

use indexmap::IndexMap;

use crate::http::collection::{Collection, RequestId, RequestRef};
use crate::http::environment::Environments;

use self::collection::FolderId;
use self::environment::EnvironmentKey;

pub mod collection;
pub mod environment;
pub mod request;

crate::new_id_type! {
    pub struct CollectionKey;
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct KeyValue {
    pub disabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct KeyValList(Vec<KeyValue>);

impl KeyValList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(vals: Vec<KeyValue>) -> Self {
        Self(vals)
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &KeyValue> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn extend(&mut self, vals: KeyValList) {
        self.0.extend(vals.0);
    }
}

impl IntoIterator for KeyValList {
    type Item = KeyValue;
    type IntoIter = std::vec::IntoIter<KeyValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct KeyFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct KeyFileList(Vec<KeyFile>);

impl KeyFileList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from(vals: Vec<KeyFile>) -> Self {
        Self(vals)
    }

    pub fn iter(&self) -> impl Iterator<Item = &KeyFile> {
        self.0.iter()
    }
}

impl IntoIterator for KeyFileList {
    type Item = KeyFile;
    type IntoIter = std::vec::IntoIter<KeyFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectionRequest(pub CollectionKey, pub RequestId);

#[derive(Debug, Default)]
pub struct Collections {
    entries: IndexMap<CollectionKey, Collection>,
    pub dirty: bool,
}

impl Collections {
    pub fn with_collection_mut<F, R>(&mut self, key: CollectionKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut Collection) -> R,
    {
        self.get_mut(key).map(f)
    }

    pub fn with_collection<F, R>(&self, key: CollectionKey, f: F) -> Option<R>
    where
        F: FnOnce(&Collection) -> R,
    {
        self.entries.get(&key).map(f)
    }

    pub fn iter(&self) -> impl Iterator<Item = (CollectionKey, &Collection)> {
        self.entries.iter().map(|(k, v)| (*k, v))
    }

    pub fn get_ref(&self, cr: CollectionRequest) -> Option<&RequestRef> {
        self.entries.get(&cr.0).and_then(|c| c.get_ref(cr.1))
    }

    pub fn get(&self, key: CollectionKey) -> Option<&Collection> {
        self.entries.get(&key)
    }

    pub fn get_mut(&mut self, key: CollectionKey) -> Option<&mut Collection> {
        self.dirty();
        self.entries.get_mut(&key)
    }

    pub fn insert_all(&mut self, collections: Vec<Collection>) {
        self.dirty();
        for collection in collections {
            self.entries.insert(CollectionKey::new(), collection);
        }
    }

    pub fn insert(&mut self, collection: Collection) {
        self.dirty();
        self.entries.insert(CollectionKey::new(), collection);
    }

    pub fn get_envs(&self, key: CollectionKey) -> Option<&Environments> {
        Some(&self.entries.get(&key)?.environments)
    }

    pub fn rename_collection(&mut self, col: CollectionKey, new: String) {
        if let Some(it) = self.get_mut(col) {
            it.rename(&new)
        }
    }

    pub fn rename_request(
        &mut self,
        req: CollectionRequest,
        new: String,
    ) -> Option<(PathBuf, PathBuf)> {
        self.get_mut(req.0)?.rename_request(req.1, &new)
    }

    pub fn rename_folder(
        &mut self,
        col: CollectionKey,
        id: FolderId,
        new: String,
    ) -> Option<(PathBuf, PathBuf)> {
        self.get_mut(col)?.rename_folder(id, &new)
    }

    pub fn create_collection(&mut self, name: String, path: PathBuf) -> &Collection {
        let path = path.join(&name);
        let collection = Collection::new(
            name,
            Vec::new(),
            Vec::new(),
            path,
            Environments::new(),
            None,
            Default::default(),
            Default::default(),
            Default::default(),
        );

        self.dirty();

        let key = CollectionKey::new();
        self.entries.insert(key, collection);
        self.entries
            .get(&key)
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
            .get_mut(&col)
            .and_then(|collection| collection.create_folder(name, folder_id))
    }

    pub fn remove(&mut self, col: CollectionKey) {
        self.dirty();
        self.entries.shift_remove(&col);
    }

    pub fn create_env(&mut self, col: CollectionKey, name: String) -> Option<EnvironmentKey> {
        let collection = self.get_mut(col)?;

        Some(collection.environments.create(name))
    }

    pub fn delete_request(&mut self, col: CollectionKey, req: RequestId) -> Option<PathBuf> {
        self.get_mut(col)?.delete_request(req)
    }

    pub fn create_script_in(&mut self, col: CollectionKey, name: String) -> Option<PathBuf> {
        self.get_mut(col)?.create_script(name)
    }

    pub fn get_script_path(&self, col: CollectionKey, s: &str) -> Option<PathBuf> {
        self.entries.get(&col)?.get_script_path(s)
    }
}
