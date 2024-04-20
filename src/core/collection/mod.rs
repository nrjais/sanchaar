use crate::core::collection::collection::{Collection, RequestId, RequestRef};
use slotmap::SlotMap;
use std::path::PathBuf;

pub mod collection;
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

    pub(crate) fn get_ref(&self, cr: &CollectionRequest) -> Option<&RequestRef> {
        self.entries.get(cr.0).and_then(|c| c.get_ref(cr.1))
    }

    pub(crate) fn insert(&mut self, collections: Vec<Collection>) {
        for collection in collections {
            self.entries.insert(collection);
        }
    }

    pub(crate) fn rename_request(
        &mut self,
        req: CollectionRequest,
        new: String,
    ) -> Option<(PathBuf, PathBuf)> {
        self.entries.get_mut(req.0)?.rename_request(req.1, &new)
    }
}
