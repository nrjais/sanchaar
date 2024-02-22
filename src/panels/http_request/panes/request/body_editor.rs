use crate::components::{code_editor, ContentType};
use crate::panels::http_request::panes::request::RequestPaneMsg;

use iced::widget::text_editor;
use iced::Element;

pub fn view(content: &text_editor::Content, content_type: ContentType) -> Element<RequestPaneMsg> {
    code_editor(content, content_type)
        .element()
        .map(RequestPaneMsg::BodyEditorAction)
}
