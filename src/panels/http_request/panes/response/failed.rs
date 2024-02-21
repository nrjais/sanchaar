use crate::components::icon;
use crate::panels::http_request::panes::response::executing::center_x;
use iced::widget::{container, text, Column};
use iced_aw::NerdIcon;

use crate::panels::http_request::panes::response::ResponseMsg;
use crate::state::AppState;

pub fn view(_state: &AppState) -> iced::Element<'_, ResponseMsg> {
    let error_icon = center_x(icon(NerdIcon::Error).size(60.0), 10);

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
