use iced::widget::{button, horizontal_space, pick_list, scrollable, Column, Row};
use iced::{Alignment, Element, Length, Task};

use components::{icon, icons, key_value_editor, tooltip, NerdIcon};
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
        let collection_key = tab.collection_key;
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
                if let Some(collection) = state.collections.get_mut(collection_key) {
                    for (key, env) in data.environments.iter() {
                        collection.update_environment(*key, env.into());
                    }
                    return builders::save_environments_cmd(collection, &data.deleted, || {
                        Message::Saved
                    });
                }
            }
            Message::EnvUpdate(env, update) => {
                if let Some(env) = data.environments.get_mut(&env) {
                    data.edited = true;
                    env.variables.update(update);
                }
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

fn icon_button<'a>(
    msg: &'a str,
    icn: NerdIcon,
    on_press: Message,
) -> iced::widget::Tooltip<'a, Message> {
    tooltip(
        msg,
        button(icon(icn))
            .on_press(on_press)
            .style(button::secondary),
    )
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

    let action_bar = Row::new()
        .push("Edit Environments")
        .push(horizontal_space().width(Length::FillPortion(3)))
        .push(icon_button("Create New", icons::Plus, Message::CreatNewEnv))
        .push_maybe(selected.map(|s| icon_button("Rename", icons::Pencil, Message::RenameEnv(s))))
        .push_maybe(selected.map(|s| icon_button("Delete", icons::Delete, Message::DeleteEnv(s))))
        .push_maybe(editor.edited.then_some(icon_button(
            "Save Changes",
            icons::ContentSave,
            Message::SaveEnvs,
        )))
        .push(
            pick_list(env_tabs, selected_name, Message::SelectEnv)
                .width(Length::FillPortion(1))
                .placeholder("Select Environment"),
        )
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    let editor = selected
        .and_then(|s| environments.get(&s).map(|e| (e, s)))
        .map(|(env, selected)| {
            let update_env = move |u| Message::EnvUpdate(selected, u);
            scrollable(key_value_editor(&env.variables).on_change(update_env)).width(Length::Fill)
        });

    Column::new()
        .push(action_bar)
        .push_maybe(editor)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
