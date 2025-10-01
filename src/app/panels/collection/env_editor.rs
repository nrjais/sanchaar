use iced::widget::{Column, Row, button, pick_list, scrollable, space};
use iced::{Alignment, Element, Length, Task};

use crate::components::{KeyValUpdateMsg, NerdIcon, icon, icons, key_value_editor, tooltip};
use core::http::collection::Collection;
use core::http::environment::EnvironmentKey;

use crate::commands::builders;
use crate::state::popups::{Popup, PopupNameAction};
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    SaveEnvs,
    SelectEnv(String),
    DeleteEnv(EnvironmentKey),
    EnvUpdate(EnvironmentKey, KeyValUpdateMsg),
    CreatNewEnv,
    RenameEnv(EnvironmentKey),
    Saved,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let key = state.active_tab;
        let Some(Tab::Collection(tab)) = state.tabs.get_mut(&key) else {
            return Task::none();
        };
        let Some(collection) = state.common.collections.get_mut(tab.collection_key) else {
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
                return builders::save_environments_cmd(collection, data).map(|_| Message::Saved);
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
            Message::Saved => (),
            Message::CreatNewEnv => {
                Popup::popup_name(
                    &mut state.common,
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
                    &mut state.common,
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

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let editor = &tab.env_editor;
    let environments = &editor.environments;

    let selected = editor.selected_env;

    let env_tabs: Vec<_> = environments.values().map(|env| env.name.clone()).collect();

    let selected_name = selected
        .and_then(|key| environments.get(&key))
        .map(|env| env.name.clone());

    let action_bar = Row::new()
        .push("Edit Environments")
        .push(space::horizontal().width(Length::FillPortion(3)))
        .push(icon_button("Create New", icons::Plus, Message::CreatNewEnv))
        .push(selected.map(|s| icon_button("Rename", icons::Pencil, Message::RenameEnv(s))))
        .push(selected.map(|s| icon_button("Delete", icons::Delete, Message::DeleteEnv(s))))
        .push(editor.edited.then_some(icon_button(
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

    let vars = col.dotenv_env_chain().all_var_set();
    let editor = selected
        .and_then(|s| environments.get(&s).map(|e| (e, s)))
        .map(|(env, selected)| {
            let update_env = move |u| Message::EnvUpdate(selected, u);
            scrollable(key_value_editor(&env.variables, &vars).on_change(update_env))
                .width(Length::Fill)
        });

    Column::new()
        .push(action_bar)
        .push(editor)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
