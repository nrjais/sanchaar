use std::time::Duration;

use iced::{
    Alignment, Border, Element, Font, Length, Size, Theme,
    font::Weight,
    widget::{
        self, Container, TextInput, Tooltip, container, responsive, text, tooltip::Position, value,
    },
};

pub fn expanded<'a, M>(base: impl Into<Element<'a, M>>) -> Container<'a, M> {
    container(base).width(Length::Fill).height(Length::Fill)
}

pub fn tooltip<'a, M: 'a>(msg: &'a str, base: impl Into<Element<'a, M>>) -> Tooltip<'a, M> {
    widget::tooltip(
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
        Position::FollowCursor,
    )
    .delay(Duration::from_millis(800))
}

pub fn text_input<'a, M: Clone + 'a>(
    placeholder: &str,
    value: &str,
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

pub fn ellipsis<'a, M: 'a>(txt: &'a str, multiple: f32, size: f32) -> Element<'a, M> {
    responsive(move |s: Size| -> Element<'a, M> {
        let txt_width = txt.len() as f32 * multiple;
        if s.width > txt_width {
            text(txt)
                .size(size)
                .align_y(Alignment::Center)
                .height(Length::Shrink)
                .into()
        } else {
            let max_chars = (s.width / multiple).min(txt.len() as f32) - 2. * multiple;
            value(format!("...{}", &txt[txt.len() - max_chars as usize..]))
                .size(size)
                .align_y(Alignment::Center)
                .height(Length::Shrink)
                .into()
        }
    })
    .into()
}
