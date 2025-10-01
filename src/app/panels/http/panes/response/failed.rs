use std::sync::Arc;

use crate::app::panels::http::panes::response::ResponsePaneMsg;
use crate::components::{icon, icons};
use iced::widget::{Column, Row, container, text};

pub fn view<'a>(e: Arc<anyhow::Error>) -> iced::Element<'a, ResponsePaneMsg> {
    let error_icon = icon(icons::Error).size(60.0);

    let error_msg = Row::new()
        .push(text("Error: "))
        .push(text(e.root_cause().to_string()))
        .align_y(iced::Alignment::Center);

    Column::new()
        .push(container(error_icon).padding(10))
        .push(error_msg)
        .align_x(iced::Alignment::Center)
        .into()
}
