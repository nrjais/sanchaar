use iced::{
    widget::{container, text},
    Element,
};

#[derive(Debug, Clone)]
pub enum ResponseMsg {}
impl ResponseMsg {
    pub(crate) fn update(&self, _state: &mut crate::state::AppState) {}
}

pub(crate) fn view(_state: &crate::state::AppState) -> Element<ResponseMsg> {
    container(text("Response Pane"))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
