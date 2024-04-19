use iced::widget::{Button, Row};
use iced::{
    widget::{button, container, pick_list, row, text_input},
    Element,
};
use reqwest::Url;
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

fn parse_path_params(url: &str) -> Option<Vec<String>> {
    let url = Url::parse(url).ok()?;

    let params = url
        .path_segments()?
        .filter(|segment| segment.starts_with(':'))
        .map(|segment| segment.trim_start_matches(':'))
        .filter(|segment| !segment.is_empty())
        .map(|segments| segments.to_string())
        .collect::<Vec<String>>();

    Some(params)
}

impl UrlBarMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            UrlBarMsg::MethodChanged(method) => {
                state.active_tab_mut().request.method = method;
            }
            UrlBarMsg::UrlChanged(url) => {
                let active_tab = state.active_tab_mut();
                if let Some(params) = parse_path_params(&url) {
                    active_tab
                        .request
                        .path_params
                        .retain(|key| params.contains(&key.name));

                    for param in params {
                        if !active_tab.request.path_params.contains_key(&param) {
                            active_tab.request.path_params.insert(param);
                        }
                    }
                }

                active_tab.request.url = url;
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
