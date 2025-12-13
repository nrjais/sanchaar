pub mod undo;
use iced::widget::TextEditor;
use iced::widget::text_editor::{Status, Style, StyleFn, default};
use iced::{Pixels, Theme, widget};
use iced_core::text::highlighter;

pub use undo::ContentAction;
pub use undo::TextContent as Content;

pub fn text_editor<'a, Message>(
    content: &'a Content,
) -> TextEditor<'a, highlighter::PlainText, Message>
where
    Message: Clone,
{
    TextEditor::new(content.internal())
}

pub struct Editor<'a> {
    code: &'a Content,
    editable: bool,
    highlight: bool,
    id: Option<widget::Id>,
    text_size: Option<Pixels>,
    style: StyleFn<'a, Theme>,
    placeholder: Option<&'a str>,
}

impl<'a> Editor<'a> {
    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Sets the text size of the [`TextEditor`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = Some(size.into());
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    pub fn id(mut self, id: widget::Id) -> Self {
        self.id = Some(id);
        self
    }
}

// impl Component for Editor<'a> {}

#[derive(Debug, Clone)]
pub enum EditorMsg {
    EditorAction(ContentAction),
    Ignored,
}

// impl LineEditorMsg {
//     pub fn update(self, state: &mut Content) {
//         match self {
//             Self::EditorAction(action) => {
//                 let block = matches!(
//                     action,
//                     ContentAction::Action(Action::Edit(Edit::Enter))
//                         | ContentAction::Action(Action::Scroll { .. })
//                 );

//                 if !block {
//                     state.perform(action);
//                 }
//             }
//             Self::Ignored => {}
//         }
//     }
// }

pub fn editor<'a>(code: &'a Content) -> Editor<'a> {
    Editor {
        code,
        editable: true,
        highlight: true,
        placeholder: None,
        text_size: None,
        id: None,
        style: Box::new(default),
    }
}
