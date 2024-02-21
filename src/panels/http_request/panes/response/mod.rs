use crate::state::response::ResponseState;
use iced::{widget::container, Element};

mod completed;
mod executing;
mod failed;
mod idle;

#[derive(Debug, Clone)]
pub enum ResponseMsg {
    Completed(completed::CompletedMsg),
}

impl ResponseMsg {
    pub(crate) fn update(self, state: &mut crate::state::AppState) {
        match self {
            Self::Completed(msg) => {
                msg.update(state);
            }
        }
    }
}

pub(crate) fn view(state: &crate::state::AppState) -> Element<ResponseMsg> {
    let active_tab = state.active_tab();
    let res = &active_tab.response;

    let res = match res.state {
        ResponseState::Idle => idle::view(state),
        ResponseState::Executing => executing::view(state),
        ResponseState::Completed(ref result) => {
            completed::view(state, result).map(ResponseMsg::Completed)
        }
        ResponseState::Failed(_) => failed::view(state),
    };

    container(res)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
}
