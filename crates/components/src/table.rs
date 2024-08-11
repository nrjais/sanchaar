use iced::padding;
use iced::widget::{container, scrollable, text, value, Column};
use iced::{widget::Row, Element, Length};
use iced_core::text::Wrapping;

use crate::{colors, horizontal_line};

pub fn table_value<'a, M: 'a>(val: impl ToString) -> Element<'a, M> {
    value(val)
        .size(14)
        .wrapping(Wrapping::WordOrGlyph)
        .color(colors::DARK_GREY)
        .into()
}

pub fn table_str<'a, M: 'a>(val: &'a str) -> Element<'a, M> {
    text(val)
        .size(14)
        .wrapping(Wrapping::WordOrGlyph)
        .color(colors::DARK_GREY)
        .into()
}

pub fn table<'a, M: 'a, const W: usize>(
    headers: [Element<'a, M>; W],
    values: impl IntoIterator<Item = [Element<'a, M>; W]>,
    widths: [u16; W],
) -> Element<'a, M> {
    let spacing = 2;
    let mut values_col = Column::new()
        .spacing(spacing)
        .padding(padding::Padding::from([4, 12]).left(0))
        .width(Length::Fill);

    let mut row = Row::new().spacing(spacing).padding(spacing);

    for (header, w) in headers.into_iter().zip(widths.iter()) {
        row = row.push(container(header).width(Length::FillPortion(1.max(*w))));
    }
    values_col = values_col.push(row).push(horizontal_line(1));

    for vals in values {
        let mut row = Row::new().spacing(spacing).padding(spacing);

        for (val, w) in vals.into_iter().zip(widths.iter()) {
            row = row.push(container(val).width(Length::FillPortion(1.max(*w))));
        }

        values_col = values_col.push(row).push(horizontal_line(1));
    }

    container(scrollable(values_col))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
