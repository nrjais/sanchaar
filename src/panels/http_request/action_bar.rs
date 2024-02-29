use iced::widget::{
    horizontal_rule, horizontal_space, pick_list, text, text_input, Button, Column, Row,
};
use iced::{
    theme,
    widget::{button, container},
    Element, Length,
};
use iced_aw::NerdIcon;

use crate::{components::icon, state::AppState};

#[derive(Debug, Clone)]
pub enum ActionBarMsg {
    Test,
    EditNameMode,
    UpdateName(String),
}

impl ActionBarMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            ActionBarMsg::Test => {}
            ActionBarMsg::EditNameMode => {
                let tab = state.active_tab_mut();
                tab.editing_name = !tab.editing_name;
            }
            ActionBarMsg::UpdateName(name) => {
                let tab = state.active_tab_mut();
                tab.request.name = name;
            }
        }
    }
}

fn icon_button<'a>(ico: NerdIcon, size: u16) -> Button<'a, ActionBarMsg> {
    button(container(icon(ico).size(size)).padding([0, 8]))
        .padding(0)
        .style(theme::Button::Text)
}

pub(crate) fn view(state: &AppState) -> Element<ActionBarMsg> {
    let tab = state.active_tab();
    let request = &tab.request;

    let size = 16;

    let name: Element<ActionBarMsg> = if tab.editing_name {
        text_input("Request Name", &request.name)
            .on_input(ActionBarMsg::UpdateName)
            .on_submit(ActionBarMsg::EditNameMode)
            .size(size - 2)
            .padding(2)
            .into()
    } else {
        text(&request.name).size(size).into()
    };

    let edit_name = icon_button(
        if tab.editing_name {
            NerdIcon::CheckBold
        } else {
            NerdIcon::PencilOutline
        },
        size,
    )
    .on_press(ActionBarMsg::EditNameMode);

    let envs = ["Dev", "Staging", "Prod"];

    let bar = Row::new()
        .push(name)
        .push(edit_name)
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
