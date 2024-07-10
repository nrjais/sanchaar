use crate::app::panels::http_request::panes::response::ResponsePaneMsg;
use iced::widget::{container, Column};

use crate::state::AppState;
use components::{icon, icons};

pub fn view(_state: &AppState) -> iced::Element<'_, ResponsePaneMsg> {
    Column::new()
        .push(container(icon(icons::SendUp).size(80.0)).padding(10))
        .push(iced::widget::Text::new("Send Request to view response."))
        .align_items(iced::Alignment::Center)
        .into()
}
