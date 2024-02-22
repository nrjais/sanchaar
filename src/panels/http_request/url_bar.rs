use iced::{
    widget::{button, container, pick_list, row, text_input},
    Element,
};
use iced_aw::NerdIcon;
use strum::VariantArray;

use crate::state::request;
use crate::state::response::ResponseState;
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
    pub(crate) fn update(self, state: &mut AppState) {
        let active_tab = state.active_tab_mut();
        match self {
            UrlBarMsg::MethodChanged(method) => {
                active_tab.request.method = method;
            }
            UrlBarMsg::UrlChanged(url) => {
                active_tab.request.url = url;
            }
            UrlBarMsg::SendRequest => {
                if let ResponseState::Executing(key) = active_tab.response.state {
                    state.cancel_task(key);
                }
                state.commands.send_request()
            }
        }
    }
}

fn build_url(req: &request::RequestPane) -> String {
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
        button(container(icon(NerdIcon::Send)).padding([0, 12])).on_press(UrlBarMsg::SendRequest);

    row!(method, url, send)
        .height(iced::Length::Shrink)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
