#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub children: Vec<Entry>,
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
    pub children: Vec<Entry>,
    pub expanded: bool,
}
