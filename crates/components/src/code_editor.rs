use iced::widget::{component, Component};
use iced::{highlighter, Element, Font, Length};
use iced_core::text::Wrapping;

use crate::editor::{self, text_editor, ContentAction};

pub enum ContentType {
    Json,
    Text,
    XML,
}

pub struct CodeEditor<'a, M> {
    pub code: &'a editor::Content,
    pub content_type: ContentType,
    pub on_action: Option<Box<dyn Fn(CodeEditorMsg) -> M>>,
    pub editable: bool,
}

impl<'a, M: 'a> CodeEditor<'a, M> {
    pub fn on_action<F>(mut self, f: F) -> CodeEditor<'a, M>
    where
        F: 'static + Fn(CodeEditorMsg) -> M,
    {
        self.on_action = Some(Box::new(f));
        self
    }

    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }
}

impl ContentType {
    pub fn to_extension(&self) -> &'static str {
        match self {
            ContentType::Json => "json",
            ContentType::Text => "txt",
            ContentType::XML => "xml",
        }
    }
}

#[derive(Debug, Clone)]
pub enum CodeEditorMsg {
    EditorAction(ContentAction, bool),
}

impl CodeEditorMsg {
    pub fn update(self, state: &mut editor::Content) {
        match self {
            Self::EditorAction(action, editable) => {
                if editable || !action.is_edit() {
                    state.perform(action);
                }
            }
        }
    }
}

pub fn code_editor<M>(code: &editor::Content, content_type: ContentType) -> CodeEditor<'_, M> {
    CodeEditor {
        code,
        content_type,
        on_action: None,
        editable: false,
    }
}

impl<'a, M> Component<M> for CodeEditor<'a, M> {
    type State = ();
    type Event = CodeEditorMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        self.on_action.as_ref().map(|on_action| on_action(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        text_editor(&self.code)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::WordOrGlyph)
            .on_action(|ac| CodeEditorMsg::EditorAction(ac, self.editable))
            .highlight(
                self.content_type.to_extension(),
                highlighter::Theme::SolarizedDark,
            )
            .into()
    }
}

impl<'a, M: 'a> From<CodeEditor<'a, M>> for Element<'a, M> {
    fn from(val: CodeEditor<'a, M>) -> Self {
        component(val)
    }
}
