use crate::components::split::split;
use iced::padding;
use iced::widget::Column;
use iced::{Element, Task, widget::container};

use crate::state::{AppState, HttpTab, Tab};

use self::panes::{request, response};

pub mod action_bar;
pub mod panes;
pub mod url_bar;

#[derive(Debug, Clone)]
pub enum HttpTabMsg {
    Req(request::RequestPaneMsg),
    Res(response::ResponsePaneMsg),
    Url(url_bar::UrlBarMsg),
    Actions(action_bar::ActionBarMsg),
    SplitResize(f32),
}

impl HttpTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {
            HttpTabMsg::Req(msg) => msg.update(state).map(HttpTabMsg::Req),
            HttpTabMsg::Res(msg) => msg.update(state).map(HttpTabMsg::Res),
            HttpTabMsg::Url(msg) => msg.update(state).map(HttpTabMsg::Url),
            HttpTabMsg::Actions(ac) => ac.update(state).map(HttpTabMsg::Actions),
            HttpTabMsg::SplitResize(ratio) => {
                let Some(Tab::Http(tab)) = state.active_tab_mut() else {
                    return Task::none();
                };
                tab.set_split_at(ratio);
                Task::none()
            }
        }
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a HttpTab) -> Element<'a, HttpTabMsg> {
    let col = state.common.collections.get(tab.collection_key());

    let url_bar = url_bar::view(tab, col).map(HttpTabMsg::Url);
    let action_bar = col.map(|col| action_bar::view(tab, col).map(HttpTabMsg::Actions));

    let request_view = request::view(tab, col).map(HttpTabMsg::Req);
    let response_view = response::view(tab).map(HttpTabMsg::Res);
    let panes = split(
        request_view,
        response_view,
        tab.split_at,
        state.split_direction,
        HttpTabMsg::SplitResize,
    )
    .handle_width(8.)
    .line_width(2.);

    let req_res = container(panes).padding(padding::top(4));
    Column::new()
        .push(action_bar)
        .push(url_bar)
        .push(req_res)
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .spacing(4)
        .into()
}
