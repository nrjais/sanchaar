use iced::{
    keyboard::{self, Event},
    Task,
};

use crate::{
    app::AppMsg,
    state::{collection_tab::CollectionTab, popups::Popup, AppState, Tab},
};

#[derive(Debug, Clone)]
pub enum Message {
    Event(iced::Event),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        match self {
            Message::Event(e) => {
                if let iced::Event::Keyboard(Event::KeyPressed { key, modifiers, .. }) = e {
                    let key = key.as_ref();
                    handle_hotkeys(key, modifiers, state);
                }
            }
        }

        Task::none()
    }
}

fn handle_hotkeys(key: keyboard::Key<&str>, modifiers: keyboard::Modifiers, state: &mut AppState) {
    match key {
        keyboard::Key::Character(c) => match c {
            "t" if modifiers.command() => state.open_tab(Tab::Http(Default::default())),
            "w" if modifiers.command() => {
                if let Some(active) = state.active_tab {
                    state.close_tab(active);
                }
            }
            "," if modifiers.command() => {
                if state.popup.is_none() {
                    Popup::app_settings(state);
                }
            }
            ";" if modifiers.command() => {
                if let Some(Tab::Http(tab)) = state.active_tab() {
                    let key = tab.collection_key();
                    let collection = state.collections.get(key);
                    if let Some(collection) = collection {
                        state.open_tab(Tab::Collection(CollectionTab::new(key, collection)));
                    }
                }
            }
            _ => (),
        },
        keyboard::Key::Named(_) => (),
        keyboard::Key::Unidentified => (),
    }
}

pub fn subscription(_: &AppState) -> iced::Subscription<AppMsg> {
    iced::event::listen()
        .map(Message::Event)
        .map(AppMsg::Subscription)
}
