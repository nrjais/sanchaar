#[derive(Debug, Default)]
pub struct CreateCollectionState {
    pub name: String,
}

#[derive(Debug)]
pub enum Popup {
    CreateCollection(CreateCollectionState),
}
