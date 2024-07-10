use components::{code_editor, text_editor, ContentType};

use crate::app::panels::http_request::panes::request::RequestPaneMsg;
use iced::widget::container;
use iced::Element;

pub fn view(content: &text_editor::Content, content_type: ContentType) -> Element<RequestPaneMsg> {
    container(
        code_editor(content, content_type)
            .editable()
            .on_action(RequestPaneMsg::BodyEditorAction),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .into()
}
