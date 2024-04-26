use iced::widget::{container, mouse_area, opaque, Stack};
use iced::{Color, Element, Length};

pub fn modal<'a, Message: Clone + 'a>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Stack::with_children([
        base.into(),
        mouse_area(
            container(opaque(content))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(|_theme| container::Style {
                    background: Some(Color::BLACK.scale_alpha(0.5).into()),
                    ..container::Style::default()
                }),
        )
        .into(),
    ])
    .into()
}
