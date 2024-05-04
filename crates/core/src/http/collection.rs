use crate::{http::environment::Environments, persistence::REQUESTS};
use std::{ops::Not, path::PathBuf};

use crate::new_id_type;

use super::environment::{Environment, EnvironmentKey};

new_id_type! {
    pub struct RequestId;
    pub struct FolderId;
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub entries: Vec<Entry>,
    pub path: PathBuf,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub struct RequestRef {
    pub id: RequestId,
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Entry {
    Item(RequestRef),
    Folder(Folder),
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub expanded: bool,
    pub environments: Environments,
    pub active_environment: Option<EnvironmentKey>,
}

impl Collection {
    pub fn new(
        name: String,
        entries: Vec<Entry>,
        path: PathBuf,
        environments: Environments,
    ) -> Self {
        let active_environment = environments.entries().next().map(|e| e.0).copied();
        Self {
            name,
            entries,
            path,
            environments,
            expanded: false,
            active_environment,
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
        self.folder_mut(id)
            .map(|folder| folder.expanded = !folder.expanded);
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

    pub fn rename_request(&mut self, id: RequestId, name: &str) -> Option<(PathBuf, PathBuf)> {
        for entry in self.iter_mut() {
            if let Entry::Item(item) = entry {
                if item.id == id {
                    let old_path = item.path.clone();
                    let new_path = item.path.with_file_name(format!("{name}.toml"));
                    item.name = name.to_string();
                    item.path = new_path.clone();
                    return Some((old_path, new_path));
                }
            }
        }
        None
    }

    pub fn rename_folder(&mut self, id: FolderId, name: &str) -> Option<(PathBuf, PathBuf)> {
        for entry in self.iter_mut() {
            if let Entry::Folder(item) = entry {
                if item.id == id {
                    let old_path = item.path.clone();
                    let new_path = item.path.with_file_name(name);
                    item.name = name.to_string();
                    item.path = new_path.clone();
                    return Some((old_path, new_path));
                }
            }
        }
        None
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

    pub fn delete_folder(&mut self, folder_id: FolderId) -> Option<PathBuf> {
        fn recurse<'a>(entries: &mut Vec<Entry>, id: FolderId) -> Option<PathBuf> {
            let mut path = None;

            let iter = entries.iter_mut();
            for entry in iter {
                if let Entry::Folder(folder) = entry {
                    if folder.id == id {
                        path = Some(folder.path.clone());
                        break;
                    } else {
                        return recurse(&mut folder.entries, id);
                    }
                }
            }
            if path.is_some() {
                entries.retain(|e| matches!(e, Entry::Folder(f) if f.id == id).not());
            }
            path
        }
        recurse(&mut self.entries, folder_id)
    }

    pub(crate) fn create_folder(
        &mut self,
        name: String,
        folder_id: Option<FolderId>,
    ) -> Option<PathBuf> {
        let create_entry = |name: String, path: &PathBuf| {
            let path = path.join(&name);
            (
                Entry::Folder(Folder {
                    name,
                    id: FolderId::new(),
                    entries: Vec::new(),
                    expanded: true,
                    path: path.clone(),
                }),
                path,
            )
        };

        if let Some(folder_id) = folder_id {
            let folder = self.folder_mut(folder_id)?;
            folder.expanded = true;

            let (entry, path) = create_entry(name, &folder.path);
            folder.entries.push(entry);

            Some(path)
        } else {
            let requests_path = self.path.join(REQUESTS);
            let (entry, path) = create_entry(name, &requests_path);
            self.entries.push(entry);
            self.expanded = true;
            Some(path)
        }
    }

    pub fn update_environment(&mut self, key: EnvironmentKey, env: Environment) {
        self.environments.update(key, env);
    }

    pub(crate) fn rename(&mut self, new: &str) -> Option<(PathBuf, PathBuf)> {
        self.name = new.to_string();
        let new_path = self.path.with_file_name(new);
        let old_path = self.path.clone();
        Some((old_path, new_path))
    }

    pub(crate) fn delete_request(&mut self, req: RequestId) -> Option<PathBuf> {
        fn recurse<'a>(entries: &mut Vec<Entry>, id: RequestId) -> Option<PathBuf> {
            let mut path = None;

            let iter = entries.iter_mut();
            for entry in iter {
                if let Entry::Item(request) = entry {
                    if request.id == id {
                        path = Some(request.path.clone());
                        break;
                    }
                }
            }
            if path.is_some() {
                entries.retain(|e| matches!(e, Entry::Item(r) if r.id == id).not());
            }
            path
        }
        recurse(&mut self.entries, req)
    }

    pub fn delete_environment(&mut self, key: EnvironmentKey) -> Option<Environment> {
        self.environments.remove(key)
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            name: "New Collection".to_string(),
            entries: vec![],
            path: PathBuf::new(),
            expanded: false,
            environments: Environments::new(),
            active_environment: None,
        }
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
