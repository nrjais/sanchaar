use std::borrow::Cow;

use iced::alignment::Horizontal;
use iced::widget::{button, container, horizontal_space, text, text_input, value, Column, Row};
use iced::{Command, Element, Length};

use components::{button_tab, key_value_editor, vertical_button_tabs};
use core::http::environment::EnvironmentKey;

use crate::state::popups::{EnvironmentEditorState, Popup};
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    SelectEnv(EnvironmentKey),
    NameChanged(String),
    CreateEnv,
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
            Message::Done => {
                Popup::close(state);
            }
            Message::CreateEnv => {
                let name = data.env_name.clone();
                data.env_name.clear();

                state.collections.create_env(data.col, name);
            }
        }
        Command::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Environment Editor")
}

pub fn done() -> Option<Message> {
    Some(Message::Done)
}

fn create_env_view(data: &EnvironmentEditorState) -> Element<Message> {
    let name = Row::new()
        .push(text("Environment Name"))
        .push(horizontal_space())
        .push(
            text_input("Name", &data.env_name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .align_items(iced::Alignment::Center)
        .spacing(4);

    Column::new()
        .push(text("No environments found, Create one!"))
        .push(name)
        .push(
            container(
                button("Add")
                    .style(button::secondary)
                    .padding([2, 4])
                    .on_press(Message::CreateEnv),
            )
            .align_x(Horizontal::Right)
            .width(Length::Fill),
        )
        .spacing(8)
        .width(350)
        .into()
}

pub fn view<'a>(state: &'a AppState, data: &'a EnvironmentEditorState) -> Element<'a, Message> {
    let environments = state.collections.get_envs(data.col).unwrap();

    let Some(first) = environments.entries().map(|e| e.0).next() else {
        return create_env_view(&data);
    };

    let env_tabs = environments.entries().map(|(key, env)| {
        button_tab(key, {
            let name = env.name.clone();
            move || value(&name)
        })
    });

    let selected = data.selected_env.unwrap_or(first);
    Row::new()
        .push(vertical_button_tabs(
            selected,
            env_tabs,
            Message::SelectEnv,
            None,
        ))
        .push(
            Column::new()
                .push(text("Variables"))
                // .push(key_value_editor(&environments.get(selected).unwrap().variables).element())
                .spacing(8),
        )
        .spacing(8)
        .width(400)
        .into()
}
