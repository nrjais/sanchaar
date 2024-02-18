use iced::{
    widget::{column, container, row},
    Element,
};
use iced_aw::split::{Axis, Split};

use crate::state::AppState;

use self::panes::{request, response};

pub mod panes;
pub mod url_bar;

#[derive(Debug, Clone)]
pub enum HttpMsg {
    Req(request::RequestMsg),
    Res(response::ResponseMsg),
    Url(url_bar::UrlBarMsg),
    SplitPos(u16),
}
impl HttpMsg {
    pub(crate) fn update(&self, state: &mut AppState) {
        match self {
            HttpMsg::Req(msg) => msg.update(state),
            HttpMsg::Res(msg) => msg.update(state),
            HttpMsg::Url(msg) => msg.update(state),
            HttpMsg::SplitPos(pos) => {
                state.active_tab_mut().request.split_pos.replace(*pos);
            }
        }
    }
}

pub(crate) fn view(state: &AppState) -> Element<HttpMsg> {
    let request = &state.active_tab().request;

    let url_bar = url_bar::view(state).map(HttpMsg::Url);
    let request_view = request::view(state).map(HttpMsg::Req);
    let response_view = response::view(state).map(HttpMsg::Res);

    let req_res = Split::new(
        container(request_view).padding([0, 4, 0, 0]),
        container(response_view).padding([0, 0, 0, 4]),
        request.split_pos,
        request.split_axis,
        HttpMsg::SplitPos,
    )
    .height(iced::Length::Fill)
    .width(iced::Length::Fill);

    let req_res = container(req_res).padding([4, 0, 0, 0]);
    column!(url_bar, req_res)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}
