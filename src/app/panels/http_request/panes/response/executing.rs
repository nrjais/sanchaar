use iced::widget::{button, container, text, Column};
use iced::Length;

use crate::app::panels::http_request::panes::response::ResponsePaneMsg;
use crate::components::{icon, icons};
use crate::state::AppState;

pub fn center_x<'a>(
    el: impl Into<iced::Element<'a, ResponsePaneMsg>>,
    padding: u16,
) -> iced::Element<'a, ResponsePaneMsg> {
    container(el)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(padding)
        .center_x()
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
        .align_items(iced::Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink);

    container(col)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
