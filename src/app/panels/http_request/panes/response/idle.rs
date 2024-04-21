use crate::app::panels::http_request::panes::response::ResponsePaneMsg;
use iced::widget::{container, Column};

use crate::state::AppState;
use components::{icon, icons};

pub fn view(_state: &AppState) -> iced::Element<'_, ResponsePaneMsg> {
    let send_icon = container(icon(icons::SendUp).size(60.0))
        .padding(10)
        .center_x()
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink);

    Column::new()
        .push(send_icon)
        .push(iced::widget::Text::new("Send Request to view response."))
        .height(iced::Length::Shrink)
        .width(iced::Length::Shrink)
        .into()
}
