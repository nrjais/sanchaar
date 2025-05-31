use crate::app::panels::http::panes::response::ResponsePaneMsg;
use iced::widget::{container, Column};

use components::{icon, icons};

pub fn view<'a>() -> iced::Element<'a, ResponsePaneMsg> {
    Column::new()
        .push(container(icon(icons::SendUp).size(80.0)).padding(10))
        .push(iced::widget::Text::new("Send Request to view response."))
        .align_x(iced::Alignment::Center)
        .into()
}
