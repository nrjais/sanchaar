use components::{icon, key_value_editor, tooltip, KeyValList, KeyValUpdateMsg, NerdIcon};
use iced::{
    padding,
    widget::{button, horizontal_space, pick_list, scrollable, Column, Row},
    Alignment, Element, Length, Task,
};

use crate::state::{collection_tab::CollectionTab, AppState, Tab};

#[derive(Debug, Clone)]
pub enum Message {
    UpdateDefaultEnv(String),
    UpdateHeaders(KeyValUpdateMsg),
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let active_tab = state.active_tab.and_then(|key| state.tabs.get_mut(&key));
        let Some(Tab::Collection(tab)) = active_tab else {
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
                    collection.set_default_env(env);
                    tab.default_env = Some(name);
                }
            }
            Message::UpdateHeaders(msg) => {
                tab.headers.update(msg);
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

pub fn view<'a>(tab: &'a CollectionTab) -> Element<'a, Message> {
    let environments = &tab.env_editor.environments;
    let envs: Vec<_> = environments
        .iter()
        .map(|(_, env)| env.name.clone())
        .collect();

    let default_env_name = tab.default_env.as_ref();

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
            .push(default_env)
            .push(headers_view(&tab.headers))
            .spacing(8)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(padding::right(12)),
    )
    .width(Length::Fill)
    .into()
}
