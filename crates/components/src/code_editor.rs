use iced::advanced::graphics::core::Element;
use iced::highlighter::Highlighter;
use iced::widget::{component, Component};
use iced::{highlighter, Font, Length, Renderer, Theme};
use iced_core::text::Wrapping;

use crate::text_editor::{self, text_editor, ContentAction};

pub enum ContentType {
    Json,
    Text,
    XML,
}

pub struct CodeEditor<'a, M> {
    pub code: &'a text_editor::Content,
    pub content_type: ContentType,
    pub on_action: Option<Box<dyn Fn(CodeEditorMsg) -> M>>,
    pub editable: bool,
}

impl<'a, M: 'a> CodeEditor<'a, M> {
    pub fn element(self) -> Element<'a, M, Theme, Renderer> {
        component(self)
    }

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
    pub fn to_extension(&self) -> String {
        match self {
            ContentType::Json => "json".to_string(),
            ContentType::Text => "txt".to_string(),
            ContentType::XML => "xml".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CodeEditorMsg {
    EditorAction(ContentAction, bool),
}

impl CodeEditorMsg {
    pub fn update(self, state: &mut text_editor::Content) {
        match self {
            Self::EditorAction(action, editable) => {
                if editable || !action.is_edit() {
                    state.perform(action);
                }
            }
        }
    }
}

pub fn code_editor<M>(code: &text_editor::Content, content_type: ContentType) -> CodeEditor<'_, M> {
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

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
        text_editor(self.code)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::Glyph)
            .on_action(|ac| CodeEditorMsg::EditorAction(ac, self.editable))
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: highlighter::Theme::SolarizedDark,
                    extension: self.content_type.to_extension(),
                },
                |highlight, _theme| highlight.to_format(),
            )
            .into()
    }
}
