mod content;
pub mod highlighters;
mod undo_stack;
mod widget;
use iced_core::text::{self, highlighter};

pub use content::{Content, ContentAction};
pub use widget::{Action, Catalog, Edit, Motion, Status, Style, StyleFn, TextEditor, default};

pub fn text_editor<'a, Message, Theme, Renderer>(
    content: &'a Content<Renderer>,
) -> TextEditor<'a, highlighter::PlainText, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog + 'a,
    Renderer: text::Renderer,
{
    TextEditor::new(content)
}
