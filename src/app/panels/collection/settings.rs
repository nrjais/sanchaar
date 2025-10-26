use lib::http::collection::Collection;
use std::{collections::HashSet, sync::Arc, time::Duration};

use crate::components::scrollable;
use crate::components::{KeyValList, KeyValUpdateMsg, key_value_editor, text_input};
use iced::{
    Alignment, Element, Length, Task, padding,
    widget::{Column, Row, pick_list, rule, space, toggler},
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
            Message::UpdateVariables(_msg) => {
                tab.edited = true;
                // tab.variables.update(msg);
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

pub fn headers_view<'a>(vals: &'a KeyValList, vars: Arc<HashSet<String>>) -> Element<'a, Message> {
    Column::new()
        .push(rule::horizontal(2.))
        .push("Collection Headers")
        .push(key_value_editor(vals, &vars).on_change(Message::UpdateHeaders))
        .spacing(8)
        .width(Length::Fill)
        .into()
}

pub fn view<'a>(tab: &'a CollectionTab, col: &'a Collection) -> Element<'a, Message> {
    let environments = &tab.env_editor.environments;
    let envs: Vec<_> = environments.values().map(|env| env.name.clone()).collect();

    let header_vars = col.env_chain().all_var_set();
    let default_env_name = tab.default_env.as_ref();

    let default_env = Row::new()
        .push("Default Environment")
        .push(space::horizontal().width(Length::FillPortion(4)))
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
        .push(space::horizontal().width(Length::FillPortion(4)))
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
        .push(space::horizontal().width(Length::FillPortion(4)))
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
            .push(default_env)
            .push(disable_ssl)
            .push(timeout)
            .push(space::horizontal().width(Length::Fixed(8.)))
            .push(headers_view(&tab.headers, header_vars))
            .spacing(16)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(padding::right(12).top(12).bottom(12)),
    )
    .width(Length::Fill)
    .into()
}
