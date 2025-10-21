use iced::{Element, Font, Length, border, highlighter};
use iced_core::text::Wrapping;

use crate::components::editor::{self, ContentAction, Status, text_editor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Json,
    Text,
    XML,
    HTML,
    JS,
}

pub struct CodeEditor<'a> {
    pub code: &'a editor::Content,
    pub content_type: ContentType,
    pub editable: bool,
}

impl<'a> CodeEditor<'a> {
    pub fn editable(mut self) -> Self {
        self.editable = true;
        self
    }

    pub fn view(self) -> Element<'a, CodeEditorMsg> {
        text_editor(self.code)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::WordOrGlyph)
            .on_action(move |ac| CodeEditorMsg::EditorAction(ac, self.editable))
            .highlight(
                self.content_type.to_extension(),
                highlighter::Theme::SolarizedDark,
            )
            .style(|theme: &iced::Theme, status| editor::Style {
                border: match status {
                    Status::Focused { .. } => border::width(1)
                        .rounded(2)
                        .color(theme.extended_palette().primary.strong.color),
                    _ => border::width(1)
                        .rounded(2)
                        .color(theme.extended_palette().background.weak.color),
                },
                ..editor::default(theme, status)
            })
            .into()
    }

    pub fn map<M: Clone + 'a>(self, f: impl Fn(CodeEditorMsg) -> M + 'a) -> Element<'a, M> {
        self.view().map(f)
    }
}

impl ContentType {
    pub fn to_extension(&self) -> &'static str {
        match self {
            ContentType::Json => "json",
            ContentType::Text => "txt",
            ContentType::XML => "xml",
            ContentType::HTML => "html",
            ContentType::JS => "js",
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

pub fn code_editor<'a>(code: &'a editor::Content, content_type: ContentType) -> CodeEditor<'a> {
    CodeEditor {
        code,
        content_type,
        editable: false,
    }
}
