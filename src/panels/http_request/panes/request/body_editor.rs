use crate::components::{code_editor, ContentType};
use crate::panels::http_request::panes::request::RequestPaneMsg;

use iced::widget::{container, text_editor};
use iced::Element;

pub fn view(content: &text_editor::Content, content_type: ContentType) -> Element<RequestPaneMsg> {
    container(
        code_editor(content, content_type)
            .editable()
            .on_action(RequestPaneMsg::BodyEditorAction)
            .element(),
    )
    .height(iced::Length::Fill)
    .width(iced::Length::Fill)
    .into()
}
