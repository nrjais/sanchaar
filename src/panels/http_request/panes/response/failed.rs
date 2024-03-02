use crate::components::{icon, icons};
use crate::panels::http_request::panes::response::executing::center_x;
use iced::widget::{container, text, Column};

use crate::panels::http_request::panes::response::ResponsePaneMsg;
use crate::state::AppState;

pub fn view(_state: &AppState) -> iced::Element<'_, ResponsePaneMsg> {
    let error_icon = center_x(icon(icons::Error).size(60.0), 10);

    let col = Column::new()
        .push(error_icon)
        .push(text("Failed to send request."))
        .height(iced::Length::Shrink)
        .width(iced::Length::Shrink);

    container(col)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
}
