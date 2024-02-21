use iced::advanced::graphics::core::Element;
use iced::highlighter::Highlighter;
use iced::widget::text_editor::Action;
use iced::widget::{component, text_editor, Component};
use iced::{highlighter, Font, Length, Renderer, Theme};

pub enum ContentType {
    Json,
    Text,
}

pub struct CodeViewer<'a, M> {
    pub code: &'a text_editor::Content,
    pub content_type: ContentType,
    pub on_action: Option<Box<dyn Fn(CodeViewerMsg) -> M>>,
}

impl<'a, M: 'a> CodeViewer<'a, M> {
    pub fn element(self) -> Element<'a, M, Theme, Renderer> {
        component(self)
    }

    pub fn on_action<F>(mut self, f: F) -> CodeViewer<'a, M>
    where
        F: 'static + Fn(CodeViewerMsg) -> M,
    {
        self.on_action = Some(Box::new(f));
        self
    }
}

impl ContentType {
    pub fn to_extension(&self) -> String {
        match self {
            ContentType::Json => "json".to_string(),
            ContentType::Text => "txt".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CodeViewerMsg {
    EditorAction(Action),
}

impl CodeViewerMsg {
    pub fn update(self, state: &mut text_editor::Content) {
        match self {
            Self::EditorAction(action) => match action {
                Action::Edit(_) => {}
                _ => {
                    state.perform(action);
                }
            },
        }
    }
}

pub fn code_viewer<M>(code: &text_editor::Content, content_type: ContentType) -> CodeViewer<'_, M> {
    CodeViewer {
        code,
        content_type,
        on_action: None,
    }
}

impl<'a, M> Component<M> for CodeViewer<'a, M> {
    type State = ();
    type Event = CodeViewerMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        self.on_action.as_ref().map(|on_action| on_action(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event, Theme, Renderer> {
        text_editor(self.code)
            .height(Length::Fill)
            .font(Font::MONOSPACE)
            .on_action(CodeViewerMsg::EditorAction)
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
