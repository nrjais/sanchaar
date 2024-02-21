use iced::{
    widget::{container, text},
    Element,
};

mod body_viewer;

#[derive(Debug, Clone)]
pub enum ResponseMsg {
    BodyViewMessage(body_viewer::BodyViewerMsg),
}

impl ResponseMsg {
    pub(crate) fn update(self, state: &mut crate::state::AppState) {
        match self {
            Self::BodyViewMessage(msg) => {
                msg.update(state);
            }
        }
    }
}

fn response_container(body: Element<ResponseMsg>) -> Element<ResponseMsg> {
    container(body)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

pub(crate) fn view(state: &crate::state::AppState) -> Element<ResponseMsg> {
    let active_tab = state.active_tab();

    let res = if let Some(ref res) = active_tab.response.response {
        body_viewer::view(state, res).map(ResponseMsg::BodyViewMessage)
    } else {
        text("Response Pane").into()
    };

    response_container(res)
}
