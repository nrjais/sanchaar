use components::{icon, icons, key_value_editor, tooltip, KeyValList, KeyValUpdateMsg, NerdIcon};
use iced::{
    padding,
    widget::{button, horizontal_space, pick_list, scrollable, Column, Row},
    Alignment, Element, Length, Task,
};

use crate::{
    commands::builders::save_collection_cmd,
    state::{collection_tab::CollectionTab, utils::to_core_kv_list, AppState, Tab, TabKey},
};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateDefaultEnv(String),
    UpdateHeaders(KeyValUpdateMsg),
    UpdateVariables(KeyValUpdateMsg),
    SaveChanges,
    Saved(TabKey),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let active_tab = state.active_tab.and_then(|key| state.tabs.get_mut(&key));
        let Some((key, Tab::Collection(tab))) = state.active_tab.zip(active_tab) else {
            return Task::none();
        };

        match self {
            Message::UpdateDefaultEnv(name) => {
                let collection = tab.collection_key;
                let env = state
                    .collections
                    .get(collection)
                    .and_then(|col| col.environments.find_by_name(&name));

                if let Some(collection) = state.collections.get_mut(collection) {
                    tab.edited = true;
                    tab.default_env = Some(name);
                    collection.set_default_env(env);
                }
            }
            Message::UpdateHeaders(msg) => {
                tab.edited = true;
                tab.headers.update(msg);
            }
            Message::UpdateVariables(msg) => {
                tab.edited = true;
                tab.variables.update(msg);
            }
            Message::SaveChanges => {
                let collection_key = tab.collection_key;
                let Some(collection) = state.collections.get_mut(collection_key) else {
                    return Task::none();
                };

                collection.default_env = tab
                    .default_env
                    .as_ref()
                    .and_then(|name| collection.environments.find_by_name(name));
                collection.headers = to_core_kv_list(&tab.headers);
                collection.variables = to_core_kv_list(&tab.variables);

                return save_collection_cmd(state, collection_key, move || Message::Saved(key));
            }
            Message::Saved(tab_key) => {
                if let Some(Tab::Collection(tab)) = state.tabs.get_mut(&tab_key) {
                    tab.edited = false;
                }
            }
        };

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

pub fn headers_view<'a>(vals: &'a KeyValList) -> Element<'a, Message> {
    Column::new()
        .push("Collection Headers")
        .push(
            key_value_editor(vals)
                .on_change(Message::UpdateHeaders)
                .padding(padding::all(0)),
        )
        .spacing(4)
        .width(Length::Fill)
        .into()
}

pub fn variables_view<'a>(vals: &'a KeyValList) -> Element<'a, Message> {
    Column::new()
        .push("Variables")
        .push(
            key_value_editor(vals)
                .on_change(Message::UpdateVariables)
                .padding(padding::all(0)),
        )
        .spacing(4)
        .width(Length::Fill)
        .into()
}

pub fn view<'a>(tab: &'a CollectionTab) -> Element<'a, Message> {
    let environments = &tab.env_editor.environments;
    let envs: Vec<_> = environments
        .iter()
        .map(|(_, env)| env.name.clone())
        .collect();

    let default_env_name = tab.default_env.as_ref();

    let action_bar = Row::new()
        .push("Collection Settings")
        .push(horizontal_space().width(Length::FillPortion(1)))
        .push_maybe(tab.edited.then_some(icon_button(
            "Save Changes",
            icons::ContentSave,
            Message::SaveChanges,
        )))
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    let default_env = Row::new()
        .push("Default Environment")
        .push(horizontal_space().width(Length::FillPortion(4)))
        .push(
            pick_list(envs, default_env_name, Message::UpdateDefaultEnv)
                .width(Length::FillPortion(1))
                .placeholder("Default Environment"),
        )
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    scrollable(
        Column::new()
            .push(action_bar)
            .push(default_env)
            .push(variables_view(&tab.variables))
            .push(headers_view(&tab.headers))
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(padding::right(12)),
    )
    .width(Length::Fill)
    .into()
}
