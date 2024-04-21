use crate::state::{AppState, Popup};
use iced::Element;

mod create_collection;

#[derive(Clone, Debug)]
pub enum PopupMsg {
    CreateCollection(create_collection::Message),
}

impl PopupMsg {
    pub fn update(self, state: &mut AppState) {
        match self {
            Self::CreateCollection(msg) => msg.update(state),
        }
    }
}

pub fn view(state: &AppState, popup: Popup) -> Element<PopupMsg> {
    match popup {
        Popup::CreateCollection => create_collection::view(state).map(PopupMsg::CreateCollection),
    }
}
