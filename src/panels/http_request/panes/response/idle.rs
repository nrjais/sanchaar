use crate::components::icon;
use iced::widget::{container, Column};
use iced_aw::NerdIcon;

use crate::panels::http_request::panes::response::ResponseMsg;
use crate::state::AppState;

pub fn view(_state: &AppState) -> iced::Element<'_, ResponseMsg> {
    let send_icon = container(icon(NerdIcon::Send).size(60.0))
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
