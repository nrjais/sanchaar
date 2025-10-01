use crate::components::{bordered_bottom, bordered_left, bordered_right, bordered_top};
use iced::padding;
use iced::widget::pane_grid::{Axis, ResizeEvent};
use iced::widget::{Column, PaneGrid, pane_grid};
use iced::{Element, Task, widget::container};

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

fn apply_border_first<'a>(
    pane: impl Into<Element<'a, HttpTabMsg>>,
    axis: Axis,
) -> Element<'a, HttpTabMsg> {
    match axis {
        Axis::Vertical => bordered_right(BORDER_WIDTH, pane),
        Axis::Horizontal => bordered_bottom(BORDER_WIDTH, pane),
    }
}

fn apply_border_second<'a>(
    pane: impl Into<Element<'a, HttpTabMsg>>,
    axis: Axis,
) -> Element<'a, HttpTabMsg> {
    match axis {
        Axis::Vertical => bordered_left(BORDER_WIDTH, pane),
        Axis::Horizontal => bordered_top(BORDER_WIDTH, pane),
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
                apply_border_first(
                    container(request_view).padding(padding::right(4)),
                    state.split_axis,
                )
            }
            SplitState::Second => {
                let response_view = response::view(tab).map(HttpTabMsg::Res);
                apply_border_second(
                    container(response_view).padding(padding::left(4)),
                    state.split_axis,
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
        .push(action_bar)
        .push(url_bar)
        .push(req_res)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
