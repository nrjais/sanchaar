use crate::components::editor::{self};
use crate::state::request::RequestPane;
use iced::Length::{Fill, Shrink};
use iced::widget::{Button, Row, rule};
use iced::{Alignment, Border, border, clipboard, mouse};
use iced::{
    Element, Task,
    widget::{button, container, pick_list},
};
use reqwest::Url;
use strum::VariantArray;

use crate::components::{
    LineEditorMsg, NerdIcon, context_menu, icon, icons, line_editor, menu_item,
};
use lib::curl::{generate_curl_command, parse_curl_command};
use lib::http::collection::Collection;
use lib::http::request::Method;

use crate::commands::builders::{ResponseResult, save_request_cmd, send_request_cmd};
use crate::state::popups::Popup;
use crate::state::{AppState, HttpTab, Tab, TabKey};

#[derive(Debug, Clone)]
pub enum UrlBarMsg {
    MethodChanged(Method),
    UrlChanged(LineEditorMsg),
    SendRequest,
    SaveRequest,
    CopyCurl,
    Done,
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
        let active_tab = state.active_tab;
        let Some(Tab::Http(tab)) = state.tabs.get_mut(&active_tab) else {
            return Task::none();
        };

        match self {
            UrlBarMsg::MethodChanged(method) => {
                tab.request_mut().method = method;
            }
            UrlBarMsg::UrlChanged(action) => {
                action.update(&mut tab.request_mut().url_content);
                let _ = parse_curl_paste(tab.request_mut());
                tab.request_mut().clean_url();
                update_path_params(tab);
            }
            UrlBarMsg::SendRequest => {
                return send_request_cmd(&mut state.common, tab)
                    .map(move |r| UrlBarMsg::RequestResult(active_tab, r));
            }
            UrlBarMsg::SaveRequest => {
                let req_ref = state.common.collections.get_ref(tab.collection_ref);
                if let Some(req_res) = req_ref {
                    let path = req_res.path.clone();
                    return save_request_cmd(tab, path).map(|_| Self::Done);
                } else {
                    Popup::save_request(&mut state.common, active_tab);
                }
            }
            UrlBarMsg::RequestResult(tab_key, res) => {
                if let Some(Tab::Http(tab)) = state.get_tab_mut(tab_key) {
                    tab.update_response(res)
                }
            }
            UrlBarMsg::Done => (),
            UrlBarMsg::CopyCurl => {
                let collection = state.common.collections.get(tab.collection_ref.0);
                let env = collection.map(|c| c.env_chain()).unwrap_or_default();
                let curl = generate_curl_command(&tab.request().to_request(), env);
                return clipboard::write(curl);
            }
        }
        Task::none()
    }
}

fn parse_curl_paste(tab: &mut RequestPane) -> Result<(), anyhow::Error> {
    let url = tab.url_content.text();
    if !url.trim().starts_with("curl ") {
        return Ok(());
    }

    let parsed = parse_curl_command(&url)?;
    tab.update_from_curl(parsed);

    Ok(())
}

fn update_path_params(tab: &mut Box<HttpTab>) {
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

fn icon_button<'a>(ico: NerdIcon) -> Button<'a, UrlBarMsg> {
    button(
        container(icon(ico))
            .align_y(Alignment::Center)
            .height(Fill)
            .padding([0, 8]),
    )
    .style(|t, s| button::Style {
        border: border::rounded(0),
        ..button::primary(t, s)
    })
}

pub fn view<'a>(tab: &'a HttpTab, col: Option<&'a Collection>) -> Element<'a, UrlBarMsg> {
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

    let mut url = line_editor(&request.url_content)
        .placeholder("https://example.com")
        .id(request.url_id.clone())
        .style(move |t: &iced::Theme, _| editor::Style {
            border,
            ..editor::default(t, editor::Status::Active)
        });

    if let Some(col) = col {
        url = url.vars(col.env_chain().all_var_set());
    }

    let url = url.map(UrlBarMsg::UrlChanged);

    let on_press = if executing {
        None
    } else {
        Some(UrlBarMsg::SendRequest)
    };

    container(
        Row::new()
            .push(method)
            .push(url)
            .push(icon_button(icons::Send).on_press_maybe(on_press))
            .push(rule::vertical(1))
            .push(menu())
            .height(Shrink)
            .width(Fill),
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

fn menu<'a>() -> Element<'a, UrlBarMsg> {
    let base = icon_button(icons::TriangleDown)
        .on_press(UrlBarMsg::Done)
        .padding(0)
        .style(|t, s| button::Style {
            border: border::rounded(border::right(4)),
            ..button::primary(t, s)
        });
    context_menu(
        base,
        vec![
            menu_item("Save", UrlBarMsg::SaveRequest),
            menu_item("Copy Curl", UrlBarMsg::CopyCurl),
        ],
    )
    .button(mouse::Button::Left)
    .into()
}
