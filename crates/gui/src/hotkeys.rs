use iced::{
    keyboard::{self, key::Named, Event},
    Task,
};

use crate::{
    app::AppMsg,
    commands::builders::{send_request_cmd, ResponseResult},
    state::{collection_tab::CollectionTab, popups::Popup, AppState, Tab, TabKey},
};

#[derive(Debug, Clone)]
pub enum Message {
    Event(iced::Event),
    RequestResult(TabKey, ResponseResult),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        match self {
            Message::Event(e) => {
                if let iced::Event::Keyboard(Event::KeyPressed { key, modifiers, .. }) = e {
                    let key = key.as_ref();
                    return handle_hotkeys(key, modifiers, state);
                }
            }
            Message::RequestResult(key, res) => {
                if let Some(Tab::Http(tab)) = state.tabs.get_mut(&key) {
                    tab.update_response(res);
                }
            }
        }

        Task::none()
    }
}

fn handle_hotkeys(
    key: keyboard::Key<&str>,
    modifiers: keyboard::Modifiers,
    state: &mut AppState,
) -> Task<Message> {
    if !modifiers.command() {
        return Task::none();
    }

    match key {
        keyboard::Key::Character(c) => match c {
            "t" if !modifiers.shift() => state.open_tab(Tab::Http(Default::default())),
            "w" if !modifiers.shift() => {
                if let Some(active) = state.active_tab {
                    state.close_tab(active);
                }
            }
            "w" if modifiers.shift() => {
                state.close_all_tabs();
            }
            "," if !modifiers.shift() => {
                if state.popup.is_none() {
                    Popup::app_settings(state);
                }
            }
            ";" if !modifiers.shift() => {
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
        keyboard::Key::Named(Named::Enter) => {
            if let Some(tab) = state.active_tab {
                let cb = move |r| Message::RequestResult(tab, r);
                return send_request_cmd(state, tab).map(cb);
            }
        }
        _ => (),
    };

    Task::none()
}

pub fn subscription(_: &AppState) -> iced::Subscription<AppMsg> {
    iced::event::listen()
        .map(Message::Event)
        .map(AppMsg::Subscription)
}
