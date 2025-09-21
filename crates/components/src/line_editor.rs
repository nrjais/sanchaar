use std::collections::HashSet;
use std::sync::Arc;

use iced::advanced::widget;
use iced::{Element, Length, Pixels, Theme};
use iced_core::text::Wrapping;
use iced_core::text::editor::{Action, Edit};

use crate::editor::highlighters::TemplHighlighterSettings;
use crate::editor::{self, ContentAction, Status, StyleFn, highlighters, text_editor};

pub struct LineEditor<'a> {
    pub code: &'a editor::Content,
    pub editable: bool,
    pub placeholder: Option<&'a str>,
    pub var_set: Arc<HashSet<String>>,
    id: Option<widget::Id>,
    text_size: Option<Pixels>,
    style: StyleFn<'a, Theme>,
}

impl<'a> LineEditor<'a> {
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

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> editor::Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    pub fn vars(mut self, vars: Arc<HashSet<String>>) -> Self {
        self.var_set = vars;
        self
    }

    pub fn id(mut self, id: widget::Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn map<M: Clone + 'a>(self, f: impl Fn(LineEditorMsg) -> M + 'a) -> Element<'a, M> {
        self.view().map(f)
    }

    pub fn view(self) -> Element<'a, LineEditorMsg> {
        let editor = text_editor(self.code)
            .height(Length::Shrink)
            .wrapping(Wrapping::WordOrGlyph)
            .style(self.style)
            .highlight_with::<highlighters::TemplHighlighter<Arc<HashSet<String>>>>(
                TemplHighlighterSettings::new(Arc::clone(&self.var_set)),
                |f, _| *f,
            )
            .on_action(move |ac| LineEditorMsg::EditorAction(ac, self.editable));

        let editor = if let Some(placeholder) = self.placeholder {
            editor.placeholder(placeholder)
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

#[derive(Debug, Clone)]
pub enum LineEditorMsg {
    EditorAction(ContentAction, bool),
}

impl LineEditorMsg {
    pub fn update(self, state: &mut editor::Content) {
        match self {
            Self::EditorAction(action, editable) => {
                let block = matches!(
                    action,
                    ContentAction::Action(Action::Edit(Edit::Enter))
                        | ContentAction::Action(Action::Scroll { .. })
                );
                let allowed = !action.is_edit() || editable;

                if allowed && !block {
                    state.perform(action);
                }
            }
        }
    }
}

pub fn line_editor<'a>(code: &'a editor::Content) -> LineEditor<'a> {
    LineEditor {
        code,
        editable: true,
        placeholder: None,
        text_size: None,
        style: Box::new(editor::default),
        var_set: HashSet::new().into(),
        id: None,
    }
}
