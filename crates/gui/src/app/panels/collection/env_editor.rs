use iced::widget::{button, horizontal_space, pick_list, scrollable, Column, Row};
use iced::{Alignment, Element, Length, Task};

use components::{icon, icons, key_value_editor, NerdIcon};
use core::http::environment::EnvironmentKey;

use crate::commands::builders;
use crate::state::collection_tab::CollectionTab;
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    SaveEnvs,
    SelectEnv(String),
    DeleteEnv(EnvironmentKey),
    EnvUpdate(EnvironmentKey, components::KeyValUpdateMsg),
    CreatNewEnv,
    RenameEnv(EnvironmentKey),
    Saved,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let active_tab = state.active_tab.and_then(|key| state.tabs.get_mut(&key));
        let Some((key, Tab::Collection(tab))) = state.active_tab.zip(active_tab) else {
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
            Message::EnvUpdate(env, update) => {
                data.edited = true;
                let env = data
                    .environments
                    .get_mut(&env)
                    .expect("Environment not found");
                env.variables.update(update);
            }
            Message::DeleteEnv(env) => {
                data.edited = true;
                data.environments.remove(&env);
                data.deleted.push(env);
                data.selected_env = None;
            }
            Message::Saved => {
                data.edited = false;
            }
            Message::CreatNewEnv => {
                Popup::popup_name(
                    state,
                    String::new(),
                    PopupNameAction::CreateEnvironment(key),
                );
            }
            Message::RenameEnv(env_key) => {
                let name = data
                    .environments
                    .get(&env_key)
                    .map(|env| env.name.clone())
                    .unwrap_or_default();
                Popup::popup_name(
                    state,
                    name,
                    PopupNameAction::RenameEnvironment(key, env_key),
                );
            }
        }
        Task::none()
    }
}

fn icon_button<'a>(icn: NerdIcon, on_press: Message) -> iced::widget::Button<'a, Message> {
    button(icon(icn))
        .on_press(on_press)
        .style(button::secondary)
}

pub fn view<'a>(tab: &'a CollectionTab) -> Element<'a, Message> {
    let editor = &tab.env_editor;
    let environments = &editor.environments;

    let selected = editor.selected_env;

    let env_tabs: Vec<_> = environments
        .iter()
        .map(|(_, env)| env.name.clone())
        .collect();

    let selected_name = selected
        .and_then(|key| environments.get(&key))
        .map(|env| env.name.clone());

    let tab_bar = Row::new()
        .push("Environment Editor")
        .push(horizontal_space().width(Length::FillPortion(3)))
        .push(icon_button(icons::Plus, Message::CreatNewEnv))
        .push_maybe(selected.map(|s| icon_button(icons::Pencil, Message::RenameEnv(s))))
        .push_maybe(selected.map(|s| icon_button(icons::Delete, Message::DeleteEnv(s))))
        .push_maybe(
            editor
                .edited
                .then_some(icon_button(icons::ContentSave, Message::SaveEnvs)),
        )
        .push(
            pick_list(env_tabs, selected_name, Message::SelectEnv)
                .width(Length::FillPortion(1))
                .placeholder("Select Environment"),
        )
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    let editor = selected.map(|selected| {
        let env = environments.get(&selected).expect("Environment not found");
        let update_env = move |u| Message::EnvUpdate(selected, u);
        scrollable(key_value_editor(&env.variables).on_change(update_env)).width(Length::Fill)
    });

    Column::new()
        .push(tab_bar)
        .push_maybe(editor)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
