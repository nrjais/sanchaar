use iced::widget::text::Wrapping;
use iced::widget::{container, text};
use iced::{
    theme,
    widget::{Column, Row},
    Border, Element, Length, Theme,
};
use iced_aw::style::colors;

pub fn key_value_viewer<'a, M: 'a>(values: &[(&'a str, &'a str)]) -> Element<'a, M> {
    let size = 16;
    let spacing = 2;
    let values = values.iter().map(|(key, val)| {
        Row::new()
            .push(
                text(key)
                    .size(size)
                    .wrapping(Wrapping::Glyph)
                    .style(theme::Text::Color(colors::DARK_GREY))
                    .width(Length::FillPortion(2)),
            )
            .push(
                text(val)
                    .size(size)
                    .wrapping(Wrapping::Glyph)
                    .style(theme::Text::Color(colors::DARK_GREY))
                    .width(Length::FillPortion(3)),
            )
            .spacing(spacing)
            .into()
    });

    container(
        Column::with_children(values)
            .spacing(spacing)
            .width(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(8)
    .style(|theme: &Theme| container::Appearance {
        border: Border {
            color: theme.extended_palette().secondary.strong.color,
            width: 1.,
            radius: 2.into(),
        },
        ..Default::default()
    })
    .into()
}