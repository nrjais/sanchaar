use lib::http::CollectionRequest;
use lib::http::request::Request;
use std::borrow::Cow;

use iced::widget::{Column, Row, space, text, text_input};
use iced::{Element, Task};

use crate::commands::builders::{
    create_folder_cmd, create_new_request_cmd, create_script_cmd, rename_folder_cmd,
    rename_request_cmd,
};
use crate::state::popups::{Popup, PopupNameAction, PopupNameState};
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Rename(String),
    Done,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let common = &mut state.common;

        let Some(Popup::PopupName(data)) = common.popup.as_mut() else {
            return Task::none();
        };

        match self {
            Message::NameChanged(name) => {
                data.name = name;
                Task::none()
            }
            Message::Rename(name) => match data.action {
                PopupNameAction::RenameCollection(col) => {
                    common.collections.rename_collection(col, name);
                    Task::done(Message::Done)
                }
                PopupNameAction::RenameFolder(col, folder_id) => {
                    rename_folder_cmd(common, col, folder_id, name).map(|_| Message::Done)
                }
                PopupNameAction::CreateFolder(col, folder_id) => {
                    create_folder_cmd(common, col, folder_id, name).map(|_| Message::Done)
                }
                PopupNameAction::RenameRequest(col, req) => {
                    rename_request_cmd(common, CollectionRequest(col, req), name)
                        .map(|_| Message::Done)
                }
                PopupNameAction::NewRequest(col, folder) => {
                    create_new_request_cmd(common, col, folder, name, Request::default())
                        .map(|_| Message::Done)
                }
                PopupNameAction::NewScript(col) => {
                    create_script_cmd(common, col, name).map(|_| Message::Done)
                }
                PopupNameAction::CreateEnvironment(tab) => {
                    if let Some(Tab::Collection(tab)) = state.get_tab_mut(tab) {
                        tab.env_editor.create_env(name);
                    }
                    Task::done(Message::Done)
                }
                PopupNameAction::RenameEnvironment(tab, env_key) => {
                    if let Some(Tab::Collection(tab)) = state.get_tab_mut(tab)
                        && let Some(mut env) = tab.env_editor.remove_env(env_key)
                    {
                        env.name = name;
                        tab.env_editor.add_env(env);
                    }
                    Task::done(Message::Done)
                }
            },
            Message::Done => {
                state.common.popup = None;
                Task::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Enter Name")
}

pub fn done(data: &PopupNameState) -> Option<Message> {
    if data.name.is_empty() {
        None
    } else {
        Some(Message::Rename(data.name.clone()))
    }
}

pub(crate) fn view<'a>(_state: &'a AppState, data: &'a PopupNameState) -> Element<'a, Message> {
    let name = Row::new()
        .push(text("Name"))
        .push(space::horizontal())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

    Column::new().push(name).spacing(4).width(300).into()
}
