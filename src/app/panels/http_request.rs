use components::{bordered_left, bordered_right};
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::{pane_grid, PaneGrid};
use iced::{
    widget::{container, Column},
    Element,
};

use crate::state::{AppState, SplitState};

use self::panes::{request, response};

pub mod action_bar;
pub mod panes;
pub mod url_bar;

#[derive(Debug, Clone)]
pub enum HttpMsg {
    Req(request::RequestPaneMsg),
    Res(response::ResponsePaneMsg),
    Url(url_bar::UrlBarMsg),
    Actions(action_bar::ActionBarMsg),
    SplitResize(ResizeEvent),
}

impl HttpMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            HttpMsg::Req(msg) => msg.update(state),
            HttpMsg::Res(msg) => msg.update(state),
            HttpMsg::Url(msg) => msg.update(state),
            HttpMsg::SplitResize(ResizeEvent { split, ratio }) => {
                // Only allow resizing if the ratio is min 0.25 on both sides
                if ratio > 0.25 && ratio < 0.75 {
                    state.active_tab_mut().panes.resize(split, ratio);
                }
            }
            HttpMsg::Actions(ac) => ac.update(state),
        }
    }
}

pub(crate) fn view(state: &AppState) -> Element<HttpMsg> {
    let tab = state.active_tab();

    let url_bar = url_bar::view(state).map(HttpMsg::Url);
    let action_bar = action_bar::view(state).map(HttpMsg::Actions);

    let req_res = PaneGrid::new(&tab.panes, move |_, pane, _| {
        let pane = match pane {
            SplitState::First => {
                let request_view = request::view(state).map(HttpMsg::Req);
                bordered_right(2, container(request_view).padding([0, 4, 0, 0]))
            }
            SplitState::Second => {
                let response_view = response::view(state).map(HttpMsg::Res);
                bordered_left(2, container(response_view).padding([0, 0, 0, 4]))
            }
        };

        pane_grid::Content::new(pane)
    })
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .on_resize(10, HttpMsg::SplitResize);

    let req_res = container(req_res).padding([4, 0, 0, 0]).into();
    Column::with_children([action_bar, url_bar, req_res])
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
