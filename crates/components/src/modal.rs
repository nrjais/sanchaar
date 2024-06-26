use iced::widget::{center, container, mouse_area, opaque, Stack};
use iced::{Color, Element};

pub fn modal<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_press: Message,
) -> Element<'a, Message> {
    Stack::with_children([
        base.into(),
        mouse_area(center(opaque(content)).style(|_theme| container::Style {
            background: Some(Color::BLACK.scale_alpha(0.5).into()),
            ..container::Style::default()
        }))
        .on_press(on_press.clone())
        .on_right_press(on_press)
        .into(),
    ])
    .into()
}
