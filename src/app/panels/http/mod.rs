use components::{bordered_left, bordered_right};
use iced::padding;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::{pane_grid, Column, PaneGrid};
use iced::{widget::container, Element, Task};

use crate::state::{AppState, HttpTab, SplitState, Tab};

use self::panes::{request, response};

pub mod action_bar;
pub mod panes;
pub mod url_bar;

const BORDER_WIDTH: u16 = 1;

#[derive(Debug, Clone)]
pub enum HttpTabMsg {
    Req(request::RequestPaneMsg),
    Res(response::ResponsePaneMsg),
    Url(url_bar::UrlBarMsg),
    Actions(action_bar::ActionBarMsg),
    SplitResize(ResizeEvent),
}

impl HttpTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            HttpTabMsg::Req(msg) => msg.update(state).map(HttpTabMsg::Req),
            HttpTabMsg::Res(msg) => msg.update(state).map(HttpTabMsg::Res),
            HttpTabMsg::Url(msg) => msg.update(state).map(HttpTabMsg::Url),
            HttpTabMsg::Actions(ac) => ac.update(state).map(HttpTabMsg::Actions),
            HttpTabMsg::SplitResize(ResizeEvent { split, ratio }) => {
                let Some(Tab::Http(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                // Only allow resizing if the ratio is min 0.25 on both sides
                if ratio > 0.25 && ratio < 0.75 {
                    tab.panes.resize(split, ratio);
                }
                Task::none()
            }
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a HttpTab) -> Element<'a, HttpTabMsg> {
    let col = state.common.collections.get(tab.collection_key());

    let url_bar = url_bar::view(tab, col).map(HttpTabMsg::Url);
    let action_bar = col.map(|col| action_bar::view(tab, col).map(HttpTabMsg::Actions));

    let req_res = PaneGrid::new(&tab.panes, move |_, pane, _| {
        let pane = match pane {
            SplitState::First => {
                let request_view = request::view(tab, col).map(HttpTabMsg::Req);
                bordered_right(
                    BORDER_WIDTH,
                    container(request_view).padding(padding::right(4)),
                )
            }
            SplitState::Second => {
                let response_view = response::view(tab).map(HttpTabMsg::Res);
                bordered_left(
                    BORDER_WIDTH,
                    container(response_view).padding(padding::left(4)),
                )
            }
        };

        pane_grid::Content::new(pane)
    })
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .on_resize(8, HttpTabMsg::SplitResize);

    let req_res = container(req_res).padding(padding::top(4));
    Column::new()
        .push_maybe(action_bar)
        .push(url_bar)
        .push(req_res)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
