use crate::state::response::ResponseState;
use iced::{widget::center, Task, Element};

mod completed;
mod executing;
mod failed;
mod idle;

#[derive(Debug, Clone)]
pub enum ResponsePaneMsg {
    Completed(completed::CompletedMsg),
    CancelRequest,
}

impl ResponsePaneMsg {
    pub(crate) fn update(self, state: &mut crate::state::AppState) -> Task<Self> {
        match self {
            Self::Completed(msg) => msg.update(state).map(ResponsePaneMsg::Completed),
            Self::CancelRequest => {
                let res_state = &state.active_tab().response.state;
                if let ResponseState::Executing = res_state {
                    state.cancel_tab_tasks(state.active_tab);
                }
                Task::none()
            }
        }
    }
}

pub(crate) fn view(state: &crate::state::AppState) -> Element<ResponsePaneMsg> {
    let active_tab = state.active_tab();
    let res = &active_tab.response;

    let res = match res.state {
        ResponseState::Idle => idle::view(state),
        ResponseState::Executing => executing::view(state),
        ResponseState::Completed(ref result) => {
            completed::view(state, result).map(ResponsePaneMsg::Completed)
        }
        ResponseState::Failed(ref e) => failed::view(state, e.clone()),
    };

    center(res).into()
}
