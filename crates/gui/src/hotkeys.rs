use iced::{
    keyboard::{self, key::Named, Event, Key},
    Task,
};

use crate::{
    app::AppMsg,
    commands::builders::{
        save_collection_cmd, save_environments_cmd, save_request_cmd, send_request_cmd,
        ResponseResult,
    },
    state::{
        collection_tab::{CollectionTab, CollectionTabId},
        popups::Popup,
        AppState, Tab, TabKey,
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    Event(iced::Event),
    RequestResult(TabKey, ResponseResult),
    Done,
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
            Message::Done => (),
        }

        Task::none()
    }
}

fn handle_hotkeys(
    key: Key<&str>,
    modifiers: keyboard::Modifiers,
    state: &mut AppState,
) -> Task<Message> {
    if !modifiers.command() {
        return Task::none();
    }

    if state.common.popup.is_some() {
        return Task::none();
    }

    match key {
        Key::Character(c) => char_hotkeys(c, modifiers, state),
        Key::Named(Named::Enter) => {
            let key = state.active_tab;
            let Some((tab, key)) = key.and_then(|key| state.tabs.get_mut(&key)).zip(key) else {
                return Task::none();
            };

            if let Tab::Http(tab) = tab {
                let cb = move |r| Message::RequestResult(key, r);
                send_request_cmd(&mut state.common, tab).map(cb)
            } else {
                Task::none()
            }
        }
        _ => Task::none(),
    }
}

fn char_hotkeys(c: &str, modifiers: keyboard::Modifiers, state: &mut AppState) -> Task<Message> {
    match c {
        "t" if !modifiers.shift() => {
            state.open_tab(Tab::Http(Default::default()));
            Task::none()
        }
        "w" if !modifiers.shift() => {
            if let Some(active) = state.active_tab {
                state.close_tab(active);
            }
            Task::none()
        }
        "w" if modifiers.shift() => {
            state.close_all_tabs();
            Task::none()
        }
        "," if !modifiers.shift() => {
            if state.common.popup.is_none() {
                Popup::app_settings(&mut state.common);
            }
            Task::none()
        }
        ";" if !modifiers.shift() => {
            if let Some(Tab::Http(tab)) = state.active_tab() {
                let key = tab.collection_key();
                let collection = state.common.collections.get(key);
                if let Some(collection) = collection {
                    state.open_tab(Tab::Collection(CollectionTab::new(key, collection)));
                }
            }
            Task::none()
        }

        "s" if !modifiers.shift() => save_tab(state),
        _ => Task::none(),
    }
}

fn save_tab(state: &mut AppState) -> Task<Message> {
    let key = state.active_tab;
    let Some((tab, key)) = key.and_then(|key| state.tabs.get_mut(&key)).zip(key) else {
        return Task::none();
    };

    match tab {
        Tab::Http(tab) => {
            let req_ref = state
                .common
                .collections
                .get_ref(tab.collection_ref)
                .cloned();
            req_ref
                .map(|req| save_request_cmd(tab, req.path).map(|_| Message::Done))
                .unwrap_or_else(|| {
                    Popup::save_request(&mut state.common, key);
                    Task::none()
                })
        }
        Tab::Collection(tab) => {
            let collection = state.common.collections.get_mut(tab.collection_key);
            let cb = move |_| Message::Done;
            let task = match tab.tab {
                CollectionTabId::Settings => {
                    collection.map(|c| save_collection_cmd(c, tab).map(cb))
                }
                CollectionTabId::Environments => {
                    collection.map(|c| save_environments_cmd(c, &mut tab.env_editor).map(cb))
                }
            };
            task.unwrap_or(Task::none())
        }
    }
}

pub fn subscription(_: &AppState) -> iced::Subscription<AppMsg> {
    iced::event::listen()
        .map(Message::Event)
        .map(AppMsg::Subscription)
}
