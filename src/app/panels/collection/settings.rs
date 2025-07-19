use core::http::collection::Collection;
use std::{collections::HashSet, sync::Arc, time::Duration};

use components::{
    KeyValList, KeyValUpdateMsg, NerdIcon, icon, icons, key_value_editor, text_input, tooltip,
};
use iced::{
    Alignment, Element, Length, Task, padding,
    widget::{Column, Row, button, horizontal_space, pick_list, scrollable, text, toggler},
};

use crate::{
    commands::builders::save_collection_cmd,
    state::tabs::collection_tab::CollectionTab,
    state::{AppState, Tab},
};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateDefaultEnv(String),
    UpdateHeaders(KeyValUpdateMsg),
    UpdateVariables(KeyValUpdateMsg),
    SaveChanges,
    Saved,
    DisableSSL(bool),
    UpdateTimeout(String),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let Some(Tab::Collection(tab)) = state.tabs.get_mut(&state.active_tab) else {
            return Task::none();
        };
        let Some(collection) = state.common.collections.get_mut(tab.collection_key) else {
            return Task::none();
        };

        match self {
            Message::UpdateDefaultEnv(name) => {
                let env = collection.environments.find_by_name(&name);
                tab.edited = true;
                tab.default_env = Some(name);
                collection.set_default_env(env);
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
                return save_collection_cmd(collection, tab).map(move |_| Message::Saved);
            }
            Message::DisableSSL(disabled) => {
                tab.edited = true;
                tab.disable_ssl = disabled;
            }
            Message::UpdateTimeout(update) => {
                if let Ok(millis) = update.parse() {
                    tab.edited = true;
                    tab.timeout_str = update;
                    tab.timeout = Duration::from_millis(millis);
                }
            }
            Message::Saved => (),
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
            .padding([1, 6])
            .on_press(on_press)
            .style(button::secondary),
    )
}

pub fn headers_view<'a>(vals: &'a KeyValList, vars: Arc<HashSet<String>>) -> Element<'a, Message> {
    Column::new()
        .push("Collection Headers")
        .push(
            key_value_editor(vals, &vars)
                .padding(padding::all(0))
                .on_change(Message::UpdateHeaders),
        )
        .spacing(4)
        .width(Length::Fill)
        .into()
}

pub fn variables_view<'a>(
    vals: &'a KeyValList,
    vars: Arc<HashSet<String>>,
) -> Element<'a, Message> {
    Column::new()
        .push("Collection Variables")
        .push(
            key_value_editor(vals, &vars)
                .padding(padding::all(0))
                .on_change(Message::UpdateVariables),
        )
        .spacing(4)
        .width(Length::Fill)
        .into()
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let environments = &tab.env_editor.environments;
    let envs: Vec<_> = environments.values().map(|env| env.name.clone()).collect();

    let header_vars = col.env_chain().all_var_set();
    let collection_vars = col.collection_env_chain().all_var_set();
    let default_env_name = tab.default_env.as_ref();

    let action_bar = Row::new()
        .push(text("Collection Settings").size(18))
        .push(horizontal_space().width(Length::FillPortion(1)))
        .push(tab.edited.then_some(icon_button(
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

    let disable_ssl = Row::new()
        .push("Disable SSL Certificate Verification")
        .push(horizontal_space().width(Length::FillPortion(4)))
        .push(
            toggler(tab.disable_ssl)
                .on_toggle(Message::DisableSSL)
                .size(20),
        )
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    let timeout = Row::new()
        .push("Default Timeout (ms)")
        .push(horizontal_space().width(Length::FillPortion(4)))
        .push(text_input(
            "Millis",
            &tab.timeout_str,
            Message::UpdateTimeout,
        ))
        .spacing(4)
        .width(Length::Fill)
        .align_y(Alignment::Center);

    scrollable(
        Column::new()
            .push(action_bar)
            .push(default_env)
            .push(disable_ssl)
            .push(timeout)
            .push(variables_view(&tab.variables, collection_vars))
            .push(headers_view(&tab.headers, header_vars))
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(padding::right(12)),
    )
    .width(Length::Fill)
    .into()
}
