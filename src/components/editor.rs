pub mod undo;
use iced::widget::text_editor::{Status, Style, StyleFn, default};
use iced::widget::{Component, TextEditor, component};
use iced::{Element, Length, Pixels, Theme, widget};
use iced_core::text::editor::Action;
use iced_core::text::{Wrapping, highlighter};

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

pub struct Editor<'a, M: 'a> {
    code: &'a Content,
    editable: bool,
    highlight: bool,
    id: Option<widget::Id>,
    text_size: Option<Pixels>,
    style: StyleFn<'a, Theme>,
    placeholder: Option<&'a str>,
    on_action: Option<Box<dyn Fn(ContentAction) -> M + 'a>>,
}

impl<'a, M: 'a> Editor<'a, M> {
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

    pub fn on_action(mut self, on_action: impl Fn(ContentAction) -> M + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }
}

#[derive(Debug, Default)]
struct State {}

#[derive(Debug, Clone)]
pub enum EditorMsg {
    EditorAction(ContentAction),
    Ignored,
}

impl<'a, M: 'a> Component<M> for Editor<'a, M> {
    type State = State;
    type Event = EditorMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        match event {
            EditorMsg::EditorAction(action) => {
                if let Some(on_action) = self.on_action.as_ref() {
                    Some(on_action(action))
                } else {
                    None
                }
            }
            EditorMsg::Ignored => None,
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'a, EditorMsg> {
        let editor = text_editor(self.code)
            .height(Length::Shrink)
            .wrapping(Wrapping::WordOrGlyph)
            .style(|theme: &Theme, status| (self.style)(theme, status));

        let editor = if self.on_action.is_some() {
            editor.on_action(move |ac: Action| {
                if !self.editable && ac.is_edit() {
                    EditorMsg::Ignored
                } else {
                    EditorMsg::EditorAction(ContentAction::Action(ac))
                }
            })
        } else {
            editor
        };

        let editor = if let Some(placeholder) = self.placeholder {
            editor.placeholder(placeholder.to_owned())
        } else {
            editor
        };

        let editor = if let Some(size) = self.text_size {
            editor.size(size)
        } else {
            editor
        };

        let editor = if let Some(id) = self.id.as_ref() {
            editor.id(id.clone())
        } else {
            editor
        };

        editor.into()
    }
}

impl<'a, M: 'a> From<Editor<'a, M>> for Element<'a, M> {
    fn from(editor: Editor<'a, M>) -> Self {
        component(editor)
    }
}

// impl Component for Editor<'a> {}

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

pub fn editor<'a, M: 'a>(code: &'a Content) -> Editor<'a, M> {
    Editor {
        code,
        editable: true,
        highlight: true,
        placeholder: None,
        text_size: None,
        id: None,
        style: Box::new(default),
        on_action: None,
    }
}
