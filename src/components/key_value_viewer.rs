use iced::widget::{Column, container, scrollable, text_input};
use iced::{Background, Border, Theme, padding};
use iced::{Element, Length, widget::Row};

use crate::components::horizontal_line;

pub fn key_value_viewer<'a, M: Clone + 'a>(values: &[(&'a str, &'a str)]) -> Element<'a, M> {
    let size = 16;
    let spacing = 2;
    let mut values_col = Column::new()
        .spacing(spacing)
        .padding(padding::Padding::from([4, 12]).left(0))
        .width(Length::Fill);

    let text_view = |v: &str| {
        text_input("", v)
            .size(size)
            .width(Length::FillPortion(3))
            .padding([0, 4])
            .style(|t: &Theme, _s| {
                let palette = t.extended_palette();
                text_input::Style {
                    background: Background::Color(palette.background.base.color),
                    border: Border::default(),
                    icon: palette.background.weak.text,
                    placeholder: palette.secondary.base.color,
                    value: palette.background.base.text,
                    selection: palette.primary.weak.color,
                }
            })
    };

    for (key, val) in values {
        let row = Row::new()
            .push(text_view(key))
            .push(text_view(val))
            .padding(spacing as u16)
            .spacing(spacing);

        values_col = values_col.push(row).push(horizontal_line(1));
    }

    container(scrollable(values_col))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
