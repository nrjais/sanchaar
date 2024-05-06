use iced::{
    widget::{container, text, tooltip::Position, Container, Tooltip},
    Border, Element, Length, Theme,
};

pub fn expanded<'a, M>(base: impl Into<Element<'a, M>>) -> Container<'a, M> {
    container(base).width(Length::Fill).height(Length::Fill)
}

pub fn tooltip<'a, M: 'a>(msg: &'static str, base: impl Into<Element<'a, M>>) -> Tooltip<'a, M> {
    iced::widget::tooltip(
        base,
        container(text(msg))
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    border: Border {
                        width: 1.0,
                        radius: 4.0.into(),
                        color: palette.background.strong.color,
                    },
                    ..Default::default()
                }
            })
            .padding([2, 4]),
        Position::Bottom,
    )
}
