use crate::state::{response::ResponseState, AppState, HttpTab, Tab};
use iced::{widget::center, Element, Task};

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
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let active_tab = state.active_tab.zip(state.active_tab_mut());
        let Some((active_tab, Tab::Http(tab))) = active_tab else {
            return Task::none();
        };
        match self {
            Self::Completed(msg) => msg.update(tab).map(ResponsePaneMsg::Completed),
            Self::CancelRequest => {
                let res_state = &tab.response.state;
                if let ResponseState::Executing = res_state {
                    state.cancel_tab_tasks(active_tab);
                }
                Task::none()
            }
        }
    }
}

pub fn view(tab: &HttpTab) -> Element<ResponsePaneMsg> {
    let res = &tab.response;

    let res = match res.state {
        ResponseState::Idle => idle::view(),
        ResponseState::Executing => executing::view(),
        ResponseState::Completed(ref result) => {
            completed::view(tab, result).map(ResponsePaneMsg::Completed)
        }
        ResponseState::Failed(ref e) => failed::view(e.clone()),
    };

    center(res).into()
}
