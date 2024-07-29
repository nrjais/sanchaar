use components::{code_editor, editor::Content, ContentType};

use crate::app::panels::http::panes::request::RequestPaneMsg;
use iced::widget::container;
use iced::Element;

pub fn view(content: &Content, content_type: ContentType) -> Element<RequestPaneMsg> {
    container(
        code_editor(content, content_type)
            .editable()
            .on_action(RequestPaneMsg::BodyEditorAction),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .into()
}
