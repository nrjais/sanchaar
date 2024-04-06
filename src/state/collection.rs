use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub children: Vec<Entry>,
    pub path: PathBuf,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub enum Entry {
    Item(Item),
    Folder(Folder),
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<Entry>,
    pub expanded: bool,
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
}
