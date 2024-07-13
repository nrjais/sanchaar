use iced::widget::{button, center, container, text, Column};
use iced::Length;

use crate::app::panels::http_request::panes::response::ResponsePaneMsg;
use crate::state::AppState;
use components::{icon, icons};

pub fn center_x<'a>(
    el: impl Into<iced::Element<'a, ResponsePaneMsg>>,
    padding: u16,
) -> iced::Element<'a, ResponsePaneMsg> {
    container(el)
        .height(Length::Shrink)
        .padding(padding)
        .center_x(Length::Fill)
        .into()
}

pub fn view(_state: &AppState) -> iced::Element<'_, ResponsePaneMsg> {
    let cancel = center_x(
        button(container(text("Cancel").size(16.0)).padding([0, 24]))
            .style(button::danger)
            .on_press(ResponsePaneMsg::CancelRequest),
        0,
    );

    let col = Column::new()
        .push(icon(icons::DotsCircle).size(40))
        .push(text("Executing Request."))
        .push(cancel)
        .spacing(8)
        .align_x(iced::Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink);

    center(col).into()
}
