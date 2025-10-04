use iced::{
    Task,
    advanced::widget::{operate, operation::focusable::focus},
    event::{Status, listen_with},
    keyboard::{self, Event, Key, key::Named},
    mouse,
    widget::operation::focus_previous,
};

use crate::{
    app::AppMsg,
    commands::builders::{
        ResponseResult, save_collection_cmd, save_environments_cmd, save_request_cmd,
        send_request_cmd,
    },
    state::{AppState, HttpTab, Tab, TabKey, popups::Popup, tabs::collection_tab::CollectionTab},
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
            let Some(tab) = state.tabs.get_mut(&key) else {
                return Task::none();
            };

            if let Tab::Http(tab) = tab {
                let cb = move |r| Message::RequestResult(key, r);
                send_request_cmd(&mut state.common, tab).map(cb)
            } else {
                Task::none()
            }
        }
        Key::Named(Named::Escape) => focus_previous(),
        _ => Task::none(),
    }
}

fn char_hotkeys(c: &str, modifiers: keyboard::Modifiers, state: &mut AppState) -> Task<Message> {
    match c {
        "t" if !modifiers.shift() => {
            state.open_tab(Tab::Http(HttpTab::new_def()));
            Task::none()
        }
        "w" if !modifiers.shift() => {
            state.close_tab(state.active_tab);
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
        "l" if !modifiers.shift() => {
            if let Some(Tab::Http(tab)) = state.active_tab() {
                return operate(focus(tab.request().url_id.clone()));
            }
            Task::none()
        }
        _ => Task::none(),
    }
}

fn save_tab(state: &mut AppState) -> Task<Message> {
    let key = state.active_tab;
    let Some(tab) = state.tabs.get_mut(&key) else {
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
            let mut collection = state.common.collections.get_mut(tab.collection_key);
            let save_collection = collection
                .as_mut()
                .map(|c| save_collection_cmd(c, tab))
                .unwrap_or(Task::none());

            let save_environments = collection
                .as_mut()
                .map(|c| save_environments_cmd(c, tab.env_editor.get_envs_for_save()))
                .unwrap_or(Task::none());

            Task::batch([save_collection, save_environments]).map(move |_| Message::Done)
        }
        Tab::CookieStore(_) => Task::none(),
        Tab::History(_) => Task::none(),
    }
}

pub fn subscription(_: &AppState) -> iced::Subscription<AppMsg> {
    listen_with(|event, status, _window| match status {
        Status::Ignored => match event {
            iced::Event::Mouse(mouse::Event::CursorMoved { .. }) => None,
            _ => Some(event),
        },
        Status::Captured => None,
    })
    .map(Message::Event)
    .map(AppMsg::Subscription)
}
