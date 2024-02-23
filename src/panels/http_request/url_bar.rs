use iced::widget::{Button, Row};
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
    SaveRequest,
}

impl UrlBarMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            UrlBarMsg::MethodChanged(method) => {
                let active_tab = state.active_tab_mut();
                active_tab.request.method = method;
            }
            UrlBarMsg::UrlChanged(url) => {
                let active_tab = state.active_tab_mut();
                active_tab.request.url = url;
            }
            UrlBarMsg::SendRequest => state.send_request(),
            UrlBarMsg::SaveRequest => state.save_request(),
        }
    }
}

// TODO: Show query params in the url
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

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, UrlBarMsg> {
    button(container(icon(ico)).padding([0, 10]))
}

pub(crate) fn view(state: &AppState) -> Element<UrlBarMsg> {
    let request = &state.active_tab().request;

    let method = pick_list(
        Method::VARIANTS,
        Some(request.method),
        UrlBarMsg::MethodChanged,
    );

    let url = text_input("Enter Address", &request.url).on_input(UrlBarMsg::UrlChanged);

    let buttons = Row::new()
        .push(icon_button(NerdIcon::Send).on_press(UrlBarMsg::SendRequest))
        .push(icon_button(NerdIcon::ContentSave).on_press(UrlBarMsg::SaveRequest))
        .spacing(2);

    row!(method, url, buttons)
        .height(iced::Length::Shrink)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
