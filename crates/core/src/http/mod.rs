use std::path::PathBuf;

use slotmap::SlotMap;

use crate::http::collection::{Collection, RequestId, RequestRef};
use crate::http::environment::Environments;

pub mod collection;
pub mod environment;
pub mod request;

slotmap::new_key_type! {
    pub struct CollectionKey;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectionRequest(pub CollectionKey, pub RequestId);

#[derive(Debug, Default)]
pub struct Collections {
    pub entries: SlotMap<CollectionKey, Collection>,
}

impl Collections {
    pub fn on_collection_mut<F, R>(&mut self, key: CollectionKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut Collection) -> R,
    {
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
        self.entries.get_mut(key)
    }

    pub fn insert_all(&mut self, collections: Vec<Collection>) {
        for collection in collections {
            self.entries.insert(collection);
        }
    }

    pub fn insert(&mut self, collection: Collection) {
        self.entries.insert(collection);
    }

    pub fn get_envs(&self, key: CollectionKey) -> Option<&Environments> {
        Some(&self.entries.get(key)?.environments)
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
        let collection = Collection::new(name, children, path);

        let key = self.entries.insert(collection);
        self.entries
            .get(key)
            .expect("Inserted collection not found")
    }
}
