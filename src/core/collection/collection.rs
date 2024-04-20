use std::path::PathBuf;

use crate::new_id_type;

new_id_type! {
    pub struct RequestId;
}

#[derive(Debug, Clone)]
pub struct Folder {
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

    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn toggle_folder(&mut self, name: &str) {
        toggle_folder(&mut self.children, name);
    }

    pub fn rename_request(&mut self, id: RequestId, name: &str) -> Option<(PathBuf, PathBuf)> {
        for entry in self.children.iter_mut() {
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
        for entry in &self.children {
            if let Entry::Item(item) = entry {
                if item.id == id {
                    return Some(item);
                }
            }
        }
        None
    }
}

fn toggle_folder(entries: &mut [Entry], name: &str) {
    for entry in entries.iter_mut() {
        if let Entry::Folder(folder) = entry {
            if folder.name == name {
                folder.expanded = !folder.expanded;
                return;
            } else {
                toggle_folder(&mut folder.children, name);
            }
        }
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
