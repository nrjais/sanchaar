use iced::widget::{Column, Row, button};
use iced::{Element, Length, Task};

use crate::app::panels::collection::env_table;
use crate::components::Direction;
use crate::components::{LineEditorMsg, scrollable_with};
use lib::http::collection::Collection;
use lib::http::environment::EnvironmentKey;

use crate::state::popups::{Popup, PopupNameAction};
use crate::state::tabs::collection_tab::CollectionTab;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    DeleteEnv(EnvironmentKey),
    CreatNewEnv,
    RenameEnv(EnvironmentKey),
    AddVariable,
    UpdateVarValue(usize, EnvironmentKey, LineEditorMsg),
    UpdateVarName(usize, LineEditorMsg),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let key = state.active_tab;
        let Some(Tab::Collection(tab)) = state.tabs.get_mut(&key) else {
            return Task::none();
        };
        let data = &mut tab.env_editor;

        match self {
            Message::DeleteEnv(env) => {
                data.remove_env(env);
            }
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
            Message::UpdateVarValue(var, env, msg) => {
                if let Some(variable) = data.variables.get_mut(var)
                    && let Some(content) = variable.values.get_mut(&env)
                {
                    msg.update(content);
                    data.edited = true;
                }
            }
            Message::AddVariable => {
                data.add_variable();
            }
            Message::UpdateVarName(index, msg) => {
                if let Some(variable) = data.variables.get_mut(index) {
                    msg.update(&mut variable.name);
                    data.edited = true;
                }
            }
        }
        Task::none()
    }
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let actions = Row::new()
        .push(
            button("Add Variable")
                .padding([2, 4])
                .on_press(Message::AddVariable)
                .style(button::secondary),
        )
        .push(
            button("New Environment")
                .padding([2, 4])
                .on_press(Message::CreatNewEnv)
                .style(button::secondary),
        )
        .spacing(8);

    let editor = scrollable_with(env_table::view(tab, col), Direction::Both).width(Length::Fill);

    Column::new()
        .push(actions)
        .push(editor)
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([8, 0])
        .into()
}
