use iced::{
    widget::{button, container, pick_list, row, text_input},
    Element,
};
use iced_aw::NerdIcon;
use strum::VariantArray;

use crate::state::request;
use crate::{
    components::icon,
    state::{request::Method, AppState},
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
                state.active_tab_mut().request.method = *method;
            }
            UrlBarMsg::UrlChanged(url) => {
                state.active_tab_mut().request.url = url.clone();
            }
            UrlBarMsg::SendRequest => {
                let req = state.active_tab().request.clone();
                state.commands.send_request(req)
            }
        }
    }
}

fn build_url(req: &request::Request) -> String {
    let mut params = String::new();
    for kv in req.query_params.values().iter() {
        if kv.disabled || kv.name.is_empty() {
            continue;
        }

        if !params.is_empty() {
            params.push('&');
        }

        params.push_str(kv.name.as_str());
        params.push('=');
        params.push_str(kv.value.as_str());
    }

    if !params.is_empty() {
        format!("{}?{}", req.url, params)
    } else {
        req.url.clone()
    }
}

pub(crate) fn view(state: &AppState) -> Element<UrlBarMsg> {
    let request = &state.active_tab().request;

    let method = pick_list(
        Method::VARIANTS,
        Some(request.method),
        UrlBarMsg::MethodChanged,
    );

    let url =
        text_input("Enter Address", build_url(request).as_str()).on_input(UrlBarMsg::UrlChanged);

    let send =
        button(container(icon(NerdIcon::Send)).padding([0, 8])).on_press(UrlBarMsg::SendRequest);

    row!(method, url, send)
        .height(iced::Length::Shrink)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
