use iced::advanced::text::Wrapping;
use iced::widget::{container, scrollable, text};
use iced::{
    widget::{Column, Row},
    Border, Element, Length, Theme,
};

use crate::components::colors;

pub fn key_value_viewer<'a, M: 'a>(values: &[(&'a str, &'a str)]) -> Element<'a, M> {
    let size = 16;
    let spacing = 2;
    let values = values.iter().map(|(key, val)| {
        Row::new()
            .push(
                text(*key)
                    .size(size)
                    .wrapping(Wrapping::Glyph)
                    .color(colors::DARK_GREY)
                    .width(Length::FillPortion(2)),
            )
            .push(
                text(*val)
                    .size(size)
                    .wrapping(Wrapping::Glyph)
                    .color(colors::DARK_GREY)
                    .width(Length::FillPortion(3)),
            )
            .spacing(spacing)
            .into()
    });

    container(scrollable(
        Column::with_children(values)
            .spacing(spacing)
            .padding([8, 12, 8, 8])
            .width(Length::Fill),
    ))
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: theme.extended_palette().secondary.strong.color,
            width: 1.,
            radius: 2.into(),
        },
        ..Default::default()
    })
    .into()
}
