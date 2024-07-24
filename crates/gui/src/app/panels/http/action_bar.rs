use components::{icons, NerdIcon};
use core::http::collection::Collection;
use core::http::CollectionKey;
use iced::widget::{horizontal_space, pick_list, text, text_input, Button, Column, Row};
use iced::{widget::button, Element, Length, Task};

use crate::commands::builders;
use crate::state::collection_tab::CollectionTab;
use crate::state::{AppState, HttpTab, Tab};

#[derive(Debug, Clone)]
pub enum ActionBarMsg {
    SubmitNameEdit,
    UpdateName(String),
    StartNameEdit,
    OpenEnvironments(CollectionKey),
    RequestRenamed(String),
    SelectEnvironment(String),
}

impl ActionBarMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::Http(tab)) = state.active_tab_mut() else {
            return Task::none();
        };

        match self {
            ActionBarMsg::StartNameEdit => {
                tab.editing_name.replace(tab.name.clone());
                Task::none()
            }
            ActionBarMsg::SubmitNameEdit => {
                let Some(name) = tab.editing_name.take() else {
                    return Task::none();
                };

                let req = tab.collection_ref;
                builders::rename_request_cmd(state, req, name.clone(), move || {
                    ActionBarMsg::RequestRenamed(name.clone())
                })
            }
            ActionBarMsg::UpdateName(name) => {
                tab.editing_name.replace(name);
                Task::none()
            }
            ActionBarMsg::OpenEnvironments(key) => {
                if let Some(col) = state.collections.get(key) {
                    state.open_tab(Tab::Collection(CollectionTab::new(key, col)));
                }
                Task::none()
            }
            ActionBarMsg::RequestRenamed(name) => {
                tab.name = name;
                Task::none()
            }
            ActionBarMsg::SelectEnvironment(name) => {
                let key = tab.collection_key();
                if let Some(col) = state.collections.get_mut(key) {
                    col.update_active_env_by_name(&name);
                };
                Task::none()
            }
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, ActionBarMsg> {
    components::icon_button(ico, None, Some(8)).style(button::text)
}

pub fn view<'a>(tab: &'a HttpTab, col: &'a Collection) -> Element<'a, ActionBarMsg> {
    let name: Element<ActionBarMsg> = match &tab.editing_name {
        Some(name) => text_input("Request Name", name)
            .on_input(ActionBarMsg::UpdateName)
            .on_paste(ActionBarMsg::UpdateName)
            .on_submit(ActionBarMsg::SubmitNameEdit)
            .padding(2)
            .into(),
        _ => text(&tab.name).into(),
    };

    let edit_name = tab
        .editing_name
        .as_ref()
        .map(|_| icon_button(icons::CheckBold).on_press(ActionBarMsg::SubmitNameEdit))
        .unwrap_or_else(|| icon_button(icons::Pencil).on_press(ActionBarMsg::StartNameEdit));

    let bar = Row::new()
        .push(name)
        .push(edit_name)
        .push(horizontal_space())
        .push(environment_view(col, tab.collection_ref.0))
        .align_y(iced::Alignment::Center)
        .width(Length::Fill);

    Column::new().push(bar).spacing(2).into()
}

fn environment_view(col: &Collection, key: CollectionKey) -> Element<'_, ActionBarMsg> {
    let envs = col
        .environments
        .entries()
        .map(|(_, env)| &env.name)
        .collect::<Vec<_>>();

    let selected = col.get_active_environment().map(|env| &env.name);

    let picker = pick_list(envs, selected, |name| {
        ActionBarMsg::SelectEnvironment(name.to_owned())
    })
    .width(Length::Shrink)
    .padding([2, 4])
    .placeholder("No Environment");

    let settings = icon_button(icons::Gear).on_press(ActionBarMsg::OpenEnvironments(key));

    Row::new()
        .push(picker)
        .push(settings)
        .align_y(iced::Alignment::Center)
        .into()
}
