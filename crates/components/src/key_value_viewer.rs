use iced::advanced::text::Wrapping;
use iced::padding;
use iced::widget::{container, scrollable, text, Column};
use iced::{widget::Row, Element, Length};

use crate::{colors, horizontal_line};

pub fn key_value_viewer<'a, M: 'a>(values: &[(&'a str, &'a str)]) -> Element<'a, M> {
    let size = 14;
    let spacing = 2;
    let mut values_col = Column::new()
        .spacing(spacing)
        .padding(padding::Padding::from([4, 12]).left(0))
        .width(Length::Fill);

    for (key, val) in values {
        let row = Row::new()
            .push(
                text(*key)
                    .size(size)
                    .wrapping(Wrapping::WordOrGlyph)
                    .color(colors::DARK_GREY)
                    .width(Length::FillPortion(2)),
            )
            .push(
                text(*val)
                    .size(size)
                    .wrapping(Wrapping::WordOrGlyph)
                    .color(colors::DARK_GREY)
                    .width(Length::FillPortion(3)),
            )
            .padding(spacing)
            .spacing(spacing);

        values_col = values_col.push(row).push(horizontal_line(1));
    }

    container(scrollable(values_col))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
