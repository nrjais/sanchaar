use crate::state::response::ResponseState;
use iced::{widget::container, Element};

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
    pub(crate) fn update(self, state: &mut crate::state::AppState) {
        match self {
            Self::Completed(msg) => {
                msg.update(state);
            }
            Self::CancelRequest => {
                let res_state = &state.active_tab().response.state;
                if let ResponseState::Executing(task) = res_state {
                    let task = state.ctx.task_cancel_tx.remove(*task);
                    if let Some(task) = task {
                        let _ = task.send(());
                    }
                }
            }
        }
    }
}

pub(crate) fn view(state: &crate::state::AppState) -> Element<ResponsePaneMsg> {
    let active_tab = state.active_tab();
    let res = &active_tab.response;

    let res = match res.state {
        ResponseState::Idle => idle::view(state),
        ResponseState::Executing(_) => executing::view(state),
        ResponseState::Completed(ref result) => {
            completed::view(state, result).map(ResponsePaneMsg::Completed)
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
