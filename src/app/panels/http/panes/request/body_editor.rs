use crate::components::{ContentType, code_editor, editor::Content};

use crate::app::panels::http::panes::request::RequestPaneMsg;
use iced::Element;
use iced::widget::container;

pub fn view(content: &Content, content_type: ContentType) -> Element<RequestPaneMsg> {
    container(
        code_editor(content, content_type)
            .editable()
            .map(RequestPaneMsg::BodyEditorAction),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .into()
}
