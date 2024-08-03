use std::collections::HashSet;
use std::sync::Arc;

use iced::widget::{component, Component};
use iced::{Element, Length, Pixels, Theme};
use iced_core::text::editor::{Action, Edit};
use iced_core::text::Wrapping;

use crate::editor::highlighters::TemplHighlighterSettings;
use crate::editor::{self, highlighters, text_editor, ContentAction, Status, StyleFn};

pub struct LineEditor<'a, M> {
    pub code: &'a editor::Content,
    pub on_action: Option<Box<dyn Fn(LineEditorMsg) -> M>>,
    pub editable: bool,
    pub placeholder: Option<&'a str>,
    pub var_set: Arc<HashSet<String>>,
    text_size: Option<Pixels>,
    style: StyleFn<'a, Theme>,
}

impl<'a, M: 'a> LineEditor<'a, M> {
    pub fn on_action<F>(mut self, f: F) -> LineEditor<'a, M>
    where
        F: 'static + Fn(LineEditorMsg) -> M,
    {
        self.on_action = Some(Box::new(f));
        self
    }

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
}

#[derive(Debug, Clone)]
pub enum LineEditorMsg {
    EditorAction(ContentAction, bool),
}

impl LineEditorMsg {
    pub fn update(self, state: &mut editor::Content) {
        match self {
            Self::EditorAction(action, editable) => {
                let newline = matches!(action, ContentAction::Action(Action::Edit(Edit::Enter)));
                let allowed = !newline || editable;

                if allowed || !action.is_edit() {
                    state.perform(action);
                }
            }
        }
    }
}

pub fn line_editor<'a, M>(code: &'a editor::Content) -> LineEditor<'a, M> {
    LineEditor {
        code,
        on_action: None,
        editable: false,
        placeholder: None,
        text_size: None,
        style: Box::new(|t, s| editor::default(t, s)),
        var_set: HashSet::new().into(),
    }
}

impl<'a, M> Component<M> for LineEditor<'a, M> {
    type State = ();
    type Event = LineEditorMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        self.on_action.as_ref().map(|on_action| on_action(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let editor = text_editor(&self.code)
            .height(Length::Shrink)
            .wrapping(Wrapping::WordOrGlyph)
            .style(|t, s| (self.style)(t, s))
            .highlight_with::<highlighters::TemplHighlighter<Arc<HashSet<String>>>>(
                TemplHighlighterSettings::new(Arc::clone(&self.var_set)),
                |f, _| *f,
            )
            .on_action(|ac| LineEditorMsg::EditorAction(ac, self.editable));

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

        editor.into()
    }
}

impl<'a, M> From<LineEditor<'a, M>> for Element<'a, M>
where
    M: 'a,
{
    fn from(val: LineEditor<'a, M>) -> Self {
        component(val)
    }
}
