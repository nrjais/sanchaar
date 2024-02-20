use iced::widget::Column;
use iced::{
    widget::{container, text},
    Element,
};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum ResponseMsg {}

impl ResponseMsg {
    pub(crate) fn update(&self, _state: &mut crate::state::AppState) {}
}

fn response_container(body: Element<ResponseMsg>) -> Element<ResponseMsg> {
    container(body)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

fn pretty_body(body: &[u8]) -> String {
    let json = serde_json::from_slice::<Value>(body);
    if let Ok(json) = json {
        serde_json::to_string_pretty(&json).unwrap()
    } else {
        String::from_utf8_lossy(body).to_string()
    }
}

pub(crate) fn view(state: &crate::state::AppState) -> Element<ResponseMsg> {
    let active_tab = state.active_tab();

    let res = if let Some(ref res) = active_tab.response {
        Column::new()
            .push(text(format!("Status: {}", res.status)))
            .push(text(format!("Duration: {:?}", res.duration)))
            .push(text(format!("Headers: {}", res.headers.len())))
            .push(text(pretty_body(&res.body.data)))
            .spacing(4)
            .into()
    } else {
        text("Response Pane").into()
    };

    response_container(res)
}
