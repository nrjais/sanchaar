use std::collections::HashSet;
use std::sync::Arc;

use iced::advanced::widget;
use iced::widget::text_editor::{self, Status, StyleFn};
use iced::{Element, Length, Pixels, Theme};
use iced_core::text::Wrapping;
use iced_core::text::editor::{Action, Edit};

use crate::components::editor::{Content, ContentAction, text_editor};
use crate::components::highlighters::{TemplHighlighter, TemplHighlighterSettings};

pub struct LineEditor<'a> {
    pub code: &'a Content,
    pub editable: bool,
    pub placeholder: Option<&'a str>,
    pub var_set: Arc<HashSet<String>>,
    pub highlight: bool,
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

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> text_editor::Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    pub fn vars(mut self, vars: Arc<HashSet<String>>) -> Self {
        self.var_set = vars;
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

    pub fn map<M: Clone + 'a>(self, f: impl Fn(LineEditorMsg) -> M + 'a) -> Element<'a, M> {
        self.view().map(f)
    }

    pub fn view(self) -> Element<'a, LineEditorMsg> {
        let editor = text_editor(self.code)
            .height(Length::Shrink)
            .wrapping(Wrapping::WordOrGlyph)
            .style(self.style)
            .on_action(move |ac| {
                if !self.editable && ac.is_edit() {
                    LineEditorMsg::Ignored
                } else {
                    LineEditorMsg::EditorAction(ContentAction::Action(ac))
                }
            });

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

        if self.highlight {
            let settings = TemplHighlighterSettings::new(Arc::clone(&self.var_set));
            editor
                .highlight_with::<TemplHighlighter<Arc<HashSet<String>>>>(settings, |f, _| *f)
                .into()
        } else {
            editor.into()
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineEditorMsg {
    EditorAction(ContentAction),
    Ignored,
}

impl LineEditorMsg {
    pub fn update(self, state: &mut Content) {
        match self {
            Self::EditorAction(action) => {
                let block = matches!(
                    action,
                    ContentAction::Action(Action::Edit(Edit::Enter))
                        | ContentAction::Action(Action::Scroll { .. })
                );

                if !block {
                    state.perform(action);
                }
            }
            Self::Ignored => {}
        }
    }
}

pub fn line_editor<'a>(code: &'a Content) -> LineEditor<'a> {
    LineEditor {
        code,
        editable: true,
        highlight: true,
        placeholder: None,
        text_size: None,
        style: Box::new(|theme: &iced::Theme, status| match status {
            Status::Focused { .. } => {
                text_editor::default(theme, Status::Focused { is_hovered: true })
            }
            _ => text_editor::default(theme, Status::Active),
        }),
        var_set: HashSet::new().into(),
        id: None,
    }
}
