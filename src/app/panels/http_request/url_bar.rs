use iced::{
    Command,
    Element, widget::{button, container, pick_list, row},
};
use iced::widget::{Button, Row};
use log::info;
use reqwest::Url;
use serde_json::Value;
use strum::VariantArray;

use components::{icon, icons, NerdIcon};
use components::text_editor::{Content, ContentAction, line_editor};
use core::http::request::Method;

use crate::commands::builders::{ResponseResult, save_request, send_request_cmd};
use crate::state::{AppState, TabKey};
use crate::state::popups::Popup;
use crate::state::response::{BodyMode, CompletedResponse, ResponseState};

#[derive(Debug, Clone)]
pub enum UrlBarMsg {
    MethodChanged(Method),
    UrlChanged(ContentAction),
    SendRequest,
    SaveRequest,
    RequestSaved,
    RequestResult(TabKey, ResponseResult),
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

fn pretty_body(body: &[u8]) -> (String, Option<String>) {
    let raw = String::from_utf8_lossy(body).to_string();

    let json = serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|v| serde_json::to_string_pretty(&v).ok());

    (raw, json)
}

fn update_response(state: &mut AppState, tab: TabKey, result: ResponseResult) {
    match result {
        ResponseResult::Completed(res) => {
            state.cancel_tab_tasks(tab);
            let Some(tab_mut) = state.get_tab_mut(tab) else {
                return;
            };

            let (raw, pretty) = pretty_body(&res.body.data);
            tab_mut.response.state = ResponseState::Completed(CompletedResponse {
                result: res,
                content: pretty.map(|p| Content::with_text(p.as_str())),
                raw: Content::with_text(raw.as_str()),
                mode: BodyMode::Pretty,
            });
        }
        ResponseResult::Error(e) => {
            state.cancel_tab_tasks(tab);
            let active_tab = state.active_tab_mut();
            active_tab.response.state = ResponseState::Failed(e);
        }
        ResponseResult::Cancelled => {
            // Response state is already updated to idle in cancel_tasks
            state.clear_tab_tasks(tab);
        }
    }
}

impl UrlBarMsg {
    pub(crate) fn update(self, state: &mut AppState) -> Command<Self> {
        match self {
            UrlBarMsg::MethodChanged(method) => {
                state.active_tab_mut().request.method = method;
            }
            UrlBarMsg::UrlChanged(action) => {
                let active_tab = state.active_tab_mut();
                active_tab.request.url_content.perform(action);

                let url = active_tab.request.url_content.text();
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
            UrlBarMsg::SendRequest => {
                let active_tab = state.active_tab_mut();
                if let ResponseState::Executing = active_tab.response.state {
                    state.cancel_tab_tasks(state.active_tab);
                }
                let tab = state.active_tab;
                return send_request_cmd(state, state.active_tab, move |r| {
                    UrlBarMsg::RequestResult(tab, r)
                });
            }
            UrlBarMsg::SaveRequest => {
                let sel_tab = state.active_tab();
                let req_ref = state.col_req_ref(state.active_tab);
                if let Some(req_res) = req_ref {
                    return save_request(&sel_tab.request, req_res.path.clone(), |_| {
                        Self::RequestSaved
                    });
                } else {
                    state.popup = Some(Popup::save_request(state.active_tab));
                }
            }
            UrlBarMsg::RequestSaved => {
                info!("Request saved");
            }
            UrlBarMsg::RequestResult(tab, res) => update_response(state, tab, res),
        }
        Command::none()
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

    let url = line_editor(&request.url_content).on_action(UrlBarMsg::UrlChanged);

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
