use iced::widget::{
    button, horizontal_space, pick_list, scrollable, text, text_input, value, vertical_space,
    Column, Row,
};
use iced::{padding, Element, Length, Task};

use components::{button_tab, icon, icons, key_value_editor, vertical_button_tabs, vertical_line};
use core::http::environment::EnvironmentKey;

use crate::commands::builders;
use crate::state::collection_tab::{CollectionTab, EnvironmentEditorState};
use crate::state::environment::Env;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    SaveEnvs,
    SelectEnv(String),
    DeleteEnv(EnvironmentKey),
    NameChanged(String),
    CreateEnv,
    EnvUpdate(EnvironmentKey, components::KeyValUpdateMsg),
    Saved,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let active_tab = state.active_tab.and_then(|key| state.tabs.get_mut(&key));
        let Some(Tab::Collection(tab)) = active_tab else {
            return Task::none();
        };
        let data = &mut tab.env_editor;

        match self {
            Message::SelectEnv(name) => {
                for (key, env) in data.environments.iter_mut() {
                    if env.name == name {
                        data.selected_env = Some(*key);
                        break;
                    }
                }
            }
            Message::NameChanged(name) => {
                data.env_name = name;
            }
            Message::SaveEnvs => {
                let col = data.col;
                if let Some(collection) = state.collections.get_mut(col) {
                    for (key, env) in data.environments.iter() {
                        collection.update_environment(*key, env.into());
                    }
                    return builders::save_environments_cmd(collection, &data.deleted, || {
                        Message::Saved
                    });
                }
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
            Message::DeleteEnv(env) => {
                data.environments.remove(&env);
                data.deleted.push(env);
            }
            Message::Saved => {
                data.edited = false;
            }
        }
        Task::none()
    }
}

fn create_env_view(data: &EnvironmentEditorState) -> Element<Message> {
    Column::new()
        .push(text("Add new environment!"))
        .push(
            text_input("Environment Name", &data.env_name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(8)
        .width(350)
        .into()
}

pub fn view<'a>(tab: &'a CollectionTab) -> Element<'a, Message> {
    let env_editor = &tab.env_editor;
    let environments = &env_editor.environments;

    let selected = env_editor.selected_env;

    let env_tabs: Vec<_> = environments
        .iter()
        .map(|(_, env)| env.name.clone())
        .collect();

    let selected_name = selected
        .and_then(|key| environments.get(&key))
        .map(|env| env.name.clone());

    let tab_bar = Row::new()
        .push(pick_list(env_tabs, selected_name, Message::SelectEnv))
        .push(vertical_line(2));

    let editor = selected.map(|selected| {
        let env = environments.get(&selected).expect("Environment not found");

        let update_env = move |u| Message::EnvUpdate(selected, u);
        Column::new()
            .push(
                Row::new()
                    .push(text("Variables"))
                    .push(horizontal_space())
                    .padding(padding::right(8)),
            )
            .push(
                scrollable(key_value_editor(&env.variables).on_change(update_env))
                    .width(Length::Fill),
            )
            .spacing(4)
    });

    Column::new()
        .push(tab_bar)
        .push_maybe(editor)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
