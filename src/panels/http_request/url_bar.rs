use iced::{
    widget::{button, column, container, pick_list, row, text_input},
    Element, Padding,
};
use iced_aw::{graphics::icons, NerdIcon};
use strum::VariantArray;

use crate::{
    components::icon,
    state::{AppState, Method},
};

#[derive(Debug, Clone)]
pub enum UrlBarMsg {
    MethodChanged(Method),
    UrlChanged(String),
    SendRequest,
}
impl UrlBarMsg {
    pub(crate) fn update(&self, state: &mut AppState) {
        match self {
            UrlBarMsg::MethodChanged(method) => {
                state.active_tab_mut().method = *method;
            }
            UrlBarMsg::UrlChanged(url) => {
                state.active_tab_mut().url = url.clone();
            }
            UrlBarMsg::SendRequest => {
                // TODO: Send the request
            }
        }
    }
}

pub(crate) fn view(state: &AppState) -> Element<UrlBarMsg> {
    let tab = state.active_tab();

    let method = pick_list(Method::VARIANTS, Some(tab.method), move |s| {
        UrlBarMsg::MethodChanged(s)
    });
    let url = text_input("Enter Address", &tab.url).on_input(|s| UrlBarMsg::UrlChanged(s));

    let send =
        button(container(icon(NerdIcon::Send)).padding([0, 8])).on_press(UrlBarMsg::SendRequest);

    row!(method, url, send)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}
