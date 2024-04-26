use std::borrow::Cow;

use iced::widget::{container, value};
use iced::{Command, Element};

use components::{button_tab, vertical_button_tabs};
use core::http::environment::EnvironmentKey;
use core::http::CollectionKey;

use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    SelectEnv(EnvironmentKey),
}

impl Message {
    pub fn update(self, _state: &mut AppState) -> Command<Message> {
        Command::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Edit Environment")
}

pub fn done() -> Option<Message> {
    Some(Message::Done)
}

pub(crate) fn view(state: &AppState, col: CollectionKey) -> Element<Message> {
    let environments = state.collections.get_envs(col).unwrap();

    let env_tabs = environments.entries().map(|(key, env)| {
        button_tab(key, {
            let name = env.name.clone();
            move || value(&name)
        })
    });

    container(vertical_button_tabs(
        environments.entries().next().unwrap().0,
        env_tabs,
        |env| Message::SelectEnv(env),
        None,
    ))
    .into()
}
