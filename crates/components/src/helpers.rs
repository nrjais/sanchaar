use iced::{
    Border, Element, Font, Length, Theme,
    font::Weight,
    widget::{Container, TextInput, Tooltip, container, text, tooltip::Position},
};

pub fn expanded<'a, M>(base: impl Into<Element<'a, M>>) -> Container<'a, M> {
    container(base).width(Length::Fill).height(Length::Fill)
}

pub fn tooltip<'a, M: 'a>(msg: &'a str, base: impl Into<Element<'a, M>>) -> Tooltip<'a, M> {
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

pub fn text_input<'a, M: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
    on_change: impl Fn(String) -> M + 'a + Clone,
) -> TextInput<'a, M> {
    iced::widget::text_input(placeholder, value)
        .on_input(on_change.clone())
        .on_paste(on_change)
}

pub fn bold<'a, M: 'a>(txt: &'a str) -> Element<'a, M> {
    text(txt)
        .font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        })
        .into()
}
