use iced::widget::{
    horizontal_rule, horizontal_space, pick_list, text, text_input, Button, Column, Row,
};
use iced::{
    widget::{button, container},
    Command, Element, Length,
};

use crate::commands::AppCommand;
use crate::state::AppState;
use components::{icon, icons, NerdIcon};

#[derive(Debug, Clone)]
pub enum ActionBarMsg {
    Test,
    SubmitNameEdit,
    UpdateName(String),
    StartNameEdit,
}

impl ActionBarMsg {
    pub(crate) fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            ActionBarMsg::StartNameEdit => {
                let name = state
                    .col_req_ref(state.active_tab)
                    .map(|r| r.name.to_string());

                if let Some(name) = name {
                    state.active_tab_mut().editing_name.replace(name);
                }
            }
            ActionBarMsg::SubmitNameEdit => {
                let tab = state.active_tab_mut();
                if let Some((col, name)) = tab.req_ref.zip(tab.editing_name.take()) {
                    let cmd = AppCommand::RenameRequest(col, name);
                    state.commands.add(cmd);
                }
            }
            ActionBarMsg::UpdateName(name) => {
                state.active_tab_mut().editing_name.replace(name);
            }
            ActionBarMsg::Test => {}
        }
        Command::none()
    }
}

fn icon_button<'a>(ico: NerdIcon, size: u16) -> Button<'a, ActionBarMsg> {
    button(container(icon(ico).size(size)).padding([0, 8]))
        .padding(0)
        .style(button::text)
}

pub(crate) fn view(state: &AppState) -> Element<ActionBarMsg> {
    let tab = state.active_tab();
    let size = 16;

    let (name, edit_name) = match state.col_req_ref(state.active_tab) {
        Some(req_ref) => {
            let name: Element<ActionBarMsg> = if let Some(name) = &tab.editing_name {
                text_input("Request Name", name)
                    .on_input(ActionBarMsg::UpdateName)
                    .on_paste(ActionBarMsg::UpdateName)
                    .on_submit(ActionBarMsg::SubmitNameEdit)
                    .size(size - 2)
                    .padding(2)
                    .into()
            } else {
                text(&req_ref.name).size(size).into()
            };

            let edit_name = tab
                .editing_name
                .as_ref()
                .map(|_| icon_button(icons::CheckBold, size).on_press(ActionBarMsg::SubmitNameEdit))
                .unwrap_or_else(|| {
                    icon_button(icons::Pencil, size).on_press(ActionBarMsg::StartNameEdit)
                });

            (Some(name), Some(edit_name))
        }
        None => (None, None),
    };

    let envs = ["Dev", "Staging", "Prod"];

    let bar = Row::new()
        .push_maybe(name)
        .push_maybe(edit_name)
        .push(horizontal_space())
        .push(
            pick_list(envs, None::<&'static str>, |_s| ActionBarMsg::Test)
                .width(Length::Shrink)
                .text_size(size)
                .padding([2, 4])
                .placeholder("No Environment"),
        )
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);

    Column::new()
        .push(bar)
        .push(horizontal_rule(4))
        .spacing(2)
        .into()
}
