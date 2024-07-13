use components::{icons, NerdIcon};
use core::http::collection::{Collection, RequestRef};
use core::http::{CollectionKey, CollectionRequest};
use iced::widget::{horizontal_space, pick_list, text, text_input, Button, Column, Row};
use iced::{widget::button, Task, Element, Length};
use log::info;

use crate::commands::builders;
use crate::state::popups::Popup;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub enum ActionBarMsg {
    SubmitNameEdit,
    UpdateName(String),
    StartNameEdit,
    OpenEnvironments(CollectionKey),
    RequestRenamed,
    SelectEnvironment(String),
}

impl ActionBarMsg {
    pub(crate) fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            ActionBarMsg::StartNameEdit => {
                let name = state
                    .get_req_ref(state.active_tab)
                    .map(|r| r.name.to_string());

                if let Some(name) = name {
                    state.active_tab_mut().editing_name.replace(name);
                }
                Task::none()
            }
            ActionBarMsg::SubmitNameEdit => {
                let tab = state.active_tab_mut();
                let Some((col, name)) = tab.collection_ref.zip(tab.editing_name.take()) else {
                    return Task::none();
                };
                builders::rename_request_cmd(state, col, name, move || ActionBarMsg::RequestRenamed)
            }
            ActionBarMsg::UpdateName(name) => {
                state.active_tab_mut().editing_name.replace(name);
                Task::none()
            }
            ActionBarMsg::OpenEnvironments(col) => {
                Popup::environment_editor(state, col);
                Task::none()
            }
            ActionBarMsg::RequestRenamed => {
                info!("Request renamed");
                Task::none()
            }
            ActionBarMsg::SelectEnvironment(name) => {
                if let Some(col) = state.active_tab().collection_ref {
                    state.collections.update_active_env_by_name(col.0, &name);
                }
                Task::none()
            }
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, ActionBarMsg> {
    components::icon_button(ico, None, Some(8)).style(button::text)
}

pub(crate) fn view(state: &AppState) -> Element<ActionBarMsg> {
    let tab = state.active_tab();

    let collection = tab.collection_ref.and_then(
        |r| -> Option<(&Collection, &RequestRef, CollectionRequest)> {
            let col = state.collections.get(r.0)?;
            Some((col, col.get_ref(r.1)?, r))
        },
    );

    let bar = collection.map(|(collection, request, col_ref)| {
        let name: Element<ActionBarMsg> = match &tab.editing_name {
            Some(name) => text_input("Request Name", name)
                .on_input(ActionBarMsg::UpdateName)
                .on_paste(ActionBarMsg::UpdateName)
                .on_submit(ActionBarMsg::SubmitNameEdit)
                .padding(2)
                .into(),
            _ => text(&request.name).into(),
        };

        let edit_name = tab
            .editing_name
            .as_ref()
            .map(|_| icon_button(icons::CheckBold).on_press(ActionBarMsg::SubmitNameEdit))
            .unwrap_or_else(|| icon_button(icons::Pencil).on_press(ActionBarMsg::StartNameEdit));

        Row::new()
            .push(name)
            .push(edit_name)
            .push(horizontal_space())
            .push(environment_view(collection, col_ref.0))
            .align_y(iced::Alignment::Center)
            .width(Length::Fill)
    });

    Column::new().push_maybe(bar).spacing(2).into()
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
