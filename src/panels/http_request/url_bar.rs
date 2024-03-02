use iced::widget::{Button, Row};
use iced::{
    widget::{button, container, pick_list, row, text_input},
    Element,
};

use strum::VariantArray;

use crate::components::{icons, NerdIcon};
use crate::{
    components::icon,
    state::{request::Method, AppState},
};

#[derive(Debug, Clone)]
pub enum UrlBarMsg {
    MethodChanged(Method),
    UrlChanged(String),
    SendRequest,
    SaveRequest,
}

impl UrlBarMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            UrlBarMsg::MethodChanged(method) => {
                state.active_tab_mut().request.method = method;
            }
            UrlBarMsg::UrlChanged(url) => {
                state.active_tab_mut().request.url = url;
            }
            UrlBarMsg::SendRequest => state.send_request(),
            UrlBarMsg::SaveRequest => state.save_request(),
        }
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, UrlBarMsg> {
    button(container(icon(ico)).padding([0, 10]))
}

pub(crate) fn view(state: &AppState) -> Element<UrlBarMsg> {
    let tab = state.active_tab();
    let request = &tab.request;
    let executing = tab.response.is_executing();

    let method = pick_list(
        Method::VARIANTS,
        Some(request.method),
        UrlBarMsg::MethodChanged,
    );

    let url = text_input("Enter Address", &request.url).on_input(UrlBarMsg::UrlChanged);

    let on_press = if executing {
        None
    } else {
        Some(UrlBarMsg::SendRequest)
    };

    let buttons = Row::new()
        .push(icon_button(icons::Send).on_press_maybe(on_press))
        .push(icon_button(icons::ContentSave).on_press(UrlBarMsg::SaveRequest))
        .spacing(2);

    row!(method, url, buttons)
        .height(iced::Length::Shrink)
        .width(iced::Length::Fill)
        .spacing(2)
        .into()
}
