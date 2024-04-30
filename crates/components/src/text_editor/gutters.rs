use iced::widget::{column, text, Row};
use iced::Element;
use iced_core::text::Highlighter;

use crate::text_editor::TextEditor;

pub fn line_numbers<'a, H: Highlighter, M: 'a>(editor: TextEditor<'a, H, M>) -> Element<'a, M> {
    let lines = (1..=editor.content.line_count())
        .map(|i| text(i.to_string()).line_height(editor.line_height).into());

    Row::new().push(column(lines)).push(editor).into()
}
