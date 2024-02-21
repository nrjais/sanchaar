use crate::components::icon;
use iced::widget::{button, container, text, Column};
use iced::Length;
use iced_aw::NerdIcon;

use crate::panels::http_request::panes::response::ResponseMsg;
use crate::state::AppState;

pub fn center_x<'a>(
    el: impl Into<iced::Element<'a, ResponseMsg>>,
    padding: u16,
) -> iced::Element<'a, ResponseMsg> {
    container(el)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(padding)
        .center_x()
        .into()
}

pub fn view(_state: &AppState) -> iced::Element<'_, ResponseMsg> {
    let send_icon = center_x(icon(NerdIcon::SendClock).size(60.0), 4);

    let cancel = center_x(
        button(container(text("Cancel").size(16.0)).padding([0, 24])),
        0,
    );

    let col = Column::new()
        .push(send_icon)
        .push(iced::widget::Text::new("Executing Request."))
        .push(cancel)
        .spacing(8)
        .height(Length::Shrink)
        .width(Length::Shrink);

    container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
