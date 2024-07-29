use iced::widget::{vertical_rule, Button, Row};
use iced::{border, Border};
use iced::{
    widget::{button, container, pick_list},
    Element, Task,
};
use reqwest::Url;
use strum::VariantArray;

use components::text_editor::{self, line_editor, ContentAction};
use components::{icon, icons, NerdIcon};
use core::http::request::Method;

use crate::commands::builders::{save_request_cmd, send_request_cmd, ResponseResult};
use crate::state::popups::Popup;
use crate::state::{AppState, HttpTab, Tab, TabKey};

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

impl UrlBarMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let active = state.active_tab.zip(state.active_tab_mut());
        let Some((active_tab, Tab::Http(tab))) = active else {
            return Task::none();
        };

        match self {
            UrlBarMsg::MethodChanged(method) => {
                tab.request_mut().method = method;
            }
            UrlBarMsg::UrlChanged(action) => {
                tab.request_mut().url_content.perform(action);

                let url = tab.request().url_content.text();
                if let Some(params) = parse_path_params(&url) {
                    tab.request_mut()
                        .path_params
                        .retain(|key| params.contains(key.name()));

                    for param in params {
                        if !tab.request().path_params.contains_key(&param) {
                            tab.request_mut().path_params.insert(param);
                        }
                    }
                }
            }
            UrlBarMsg::SendRequest => {
                return send_request_cmd(state, active_tab)
                    .map(move |r| UrlBarMsg::RequestResult(active_tab, r));
            }
            UrlBarMsg::SaveRequest => {
                let Some(Tab::Http(tab)) = state.active_tab() else {
                    return Task::none();
                };

                let req_ref = state.collections.get_ref(tab.collection_ref);
                if let Some(req_res) = req_ref {
                    let path = req_res.path.clone();
                    return save_request_cmd(tab.request(), path).map(|_| Self::RequestSaved);
                } else {
                    Popup::save_request(state, active_tab);
                }
            }
            UrlBarMsg::RequestSaved => tab.check_dirty(),
            UrlBarMsg::RequestResult(tab, res) => {
                if let Some(Tab::Http(tab)) = state.get_tab_mut(tab) {
                    tab.update_response(res)
                }
            }
        }
        Task::none()
    }
}

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, UrlBarMsg> {
    button(container(icon(ico)).padding([0, 8])).style(|t, s| button::Style {
        border: border::rounded(0),
        ..button::primary(t, s)
    })
}

pub(crate) fn view(tab: &HttpTab) -> Element<UrlBarMsg> {
    let request = tab.request();
    let executing = tab.response.is_executing();

    let border = Border::default();

    let method = pick_list(
        Method::VARIANTS,
        Some(request.method),
        UrlBarMsg::MethodChanged,
    )
    .style(move |theme, _| pick_list::Style {
        border: border.rounded(border::left(4)),
        ..pick_list::default(theme, pick_list::Status::Active)
    });

    let url = line_editor(&request.url_content)
        .placeholder("https://example.com")
        .style(move |t: &iced::Theme, _| text_editor::Style {
            border,
            ..text_editor::default(t, text_editor::Status::Active)
        })
        .on_action(UrlBarMsg::UrlChanged);

    let on_press = if executing {
        None
    } else {
        Some(UrlBarMsg::SendRequest)
    };

    let buttons = Row::new()
        .push(icon_button(icons::Send).on_press_maybe(on_press))
        .push(vertical_rule(1))
        .push(
            icon_button(icons::ContentSave)
                .on_press(UrlBarMsg::SaveRequest)
                .style(|t, s| button::Style {
                    border: border::rounded(border::right(4)),
                    ..button::primary(t, s)
                }),
        );

    container(
        Row::new()
            .push(method)
            .push(url)
            .push(buttons)
            .height(iced::Length::Shrink)
            .width(iced::Length::Fill),
    )
    .style(|theme| {
        let base = container::bordered_box(theme);
        container::Style {
            border: base.border.width(2).rounded(4),
            ..base
        }
    })
    .padding(1)
    .into()
}
