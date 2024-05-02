use std::borrow::Cow;
use std::ops::Not;

use iced::widget::{scrollable, text, text_input, value, Column, Row};
use iced::{Command, Element};

use components::{button_tab, key_value_editor, vertical_button_tabs, vertical_line};
use core::http::environment::EnvironmentKey;

use crate::commands::builders;
use crate::state::environment::Env;
use crate::state::popups::{EnvironmentEditorState, Popup};
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    SaveEnvs,
    SelectEnv(EnvironmentKey),
    NameChanged(String),
    CreateEnv,
    EnvUpdate(EnvironmentKey, components::KeyValUpdateMsg),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(Popup::EnvironmentEditor(data)) = state.popup.as_mut() else {
            return Command::none();
        };

        match self {
            Message::SelectEnv(env) => {
                data.selected_env = Some(env);
            }
            Message::NameChanged(name) => {
                data.env_name = name;
            }
            Message::SaveEnvs => {
                let col = data.col;
                if let Some(collection) = state.collections.get_mut(col) {
                    let envs = data.environments.drain();
                    for (key, env) in envs {
                        collection.update_environment(key, env.into());
                    }
                    return builders::save_environments_cmd(collection, || Message::Done);
                }
            }
            Message::Done => {
                Popup::close(state);
            }
            Message::CreateEnv => {
                let name = data.env_name.clone();
                data.env_name.clear();
                let key = state
                    .collections
                    .create_env(data.col, name)
                    .expect("Failed to create env");
                let env = state
                    .collections
                    .get_envs(data.col)
                    .expect("Environment not found")
                    .get(key)
                    .expect("Environment not found");

                data.environments.insert(key, Env::from(env));
            }
            Message::EnvUpdate(env, update) => {
                let env = data
                    .environments
                    .get_mut(&env)
                    .expect("Environment not found");
                env.variables.update(update);
            }
        }
        Command::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Environment Editor")
}

pub fn done(data: &EnvironmentEditorState) -> Option<Message> {
    let empty = data.environments.is_empty();
    if empty {
        data.env_name.is_empty().not().then_some(Message::CreateEnv)
    } else {
        Some(Message::SaveEnvs)
    }
}

fn create_env_view(data: &EnvironmentEditorState) -> Element<Message> {
    Column::new()
        .push(text("No environments found, Create one!"))
        .push(
            text_input("Environment Name", &data.env_name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(8)
        .width(350)
        .into()
}

pub fn view<'a>(_state: &'a AppState, data: &'a EnvironmentEditorState) -> Element<'a, Message> {
    let environments = &data.environments;

    let Some(first) = environments.iter().map(|e| e.0).next() else {
        return create_env_view(data);
    };

    let selected = data.selected_env.unwrap_or(*first);
    let env = environments.get(&selected).expect("Environment not found");

    let env_tabs = environments.iter().map(|(key, env)| {
        button_tab(*key, {
            let name = env.name.clone();
            move || value(&name)
        })
    });

    let tab_bar = Row::new()
        .push(scrollable(vertical_button_tabs(
            selected,
            env_tabs,
            Message::SelectEnv,
            None,
        )))
        .push(vertical_line(2));

    let update_env = move |u| Message::EnvUpdate(selected, u);
    let editor = Column::new()
        .push(text("Variables"))
        .push(key_value_editor(&env.variables).on_change(update_env))
        .spacing(4);

    Row::new()
        .push(tab_bar)
        .push(scrollable(editor))
        .spacing(8)
        .height(400)
        .width(450)
        .into()
}
