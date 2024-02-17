use iced::{
    widget::{column, row},
    Element,
};

use crate::state::AppState;

use self::panes::{request, response};

pub mod panes;
pub mod url_bar;

#[derive(Debug, Clone)]
pub enum HttpMsg {
    Req(request::RequestMsg),
    Res(response::ResponseMsg),
    Url(url_bar::UrlBarMsg),
}
impl HttpMsg {
    pub(crate) fn update(&self, state: &mut AppState) {
        match self {
            HttpMsg::Req(msg) => msg.update(state),
            HttpMsg::Res(msg) => msg.update(state),
            HttpMsg::Url(msg) => msg.update(state),
        }
    }
}

pub(crate) fn view(state: &AppState) -> Element<HttpMsg> {
    let url_bar = url_bar::view(state).map(HttpMsg::Url);
    let request = request::view(state).map(HttpMsg::Req);
    let response = response::view(state).map(HttpMsg::Res);

    let req_res = row![request, response].spacing(10);

    column!(url_bar, req_res)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
}
