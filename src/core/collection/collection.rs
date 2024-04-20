use std::path::PathBuf;

use crate::new_id_type;

new_id_type! {
    pub struct RequestId;
    pub struct FolderId;
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub children: Vec<Entry>,
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
    pub children: Vec<Entry>,
    pub expanded: bool,
}

impl Collection {
    pub fn new(name: String, children: Vec<Entry>, path: PathBuf) -> Self {
        Self {
            name,
            children,
            path,
            expanded: false,
        }
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut {
            stack: self.children.iter_mut().collect::<Vec<_>>(),
        }
    }

    fn iter(&self) -> Iter {
        Iter {
            stack: self.children.iter().collect::<Vec<_>>(),
        }
    }

    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn toggle_folder(&mut self, id: FolderId) {
        folder_mut(&mut self.children, id).map(|folder| folder.expanded = !folder.expanded);
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
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            name: "New Collection".to_string(),
            children: vec![],
            path: PathBuf::new(),
            expanded: false,
        }
    }
}

fn folder_mut(entries: &mut Vec<Entry>, id: FolderId) -> Option<&mut Folder> {
    for entry in entries.iter_mut() {
        if let Entry::Folder(folder) = entry {
            if folder.id == id {
                return Some(folder);
            } else {
                if let Some(folder) = folder_mut(&mut folder.children, id) {
                    return Some(folder);
                }
            }
        }
    }
    None
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
                Entry::Folder(Folder { children, .. }) => {
                    self.stack.extend(children.iter_mut());
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
            Entry::Folder(Folder { children, .. }) => {
                self.stack.extend(children.iter());
                Some(node)
            }
            Entry::Item(_) => Some(node),
        };
    }
}
