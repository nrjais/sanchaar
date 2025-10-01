use iced::widget::{container, scrollable, table, text_input};
use iced::{Background, Border, Theme};
use iced::{Element, Length};

use crate::components::bold;

pub fn key_value_viewer<'a, M: Clone + 'a>(
    values: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Element<'a, M> {
    let text_view = |v: &str| {
        text_input("", v)
            .size(16)
            .width(Length::FillPortion(3))
            .padding(0)
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

    let columns = [
        table::column(bold("Name"), |(key, _): (&str, &str)| text_view(key))
            .width(Length::FillPortion(1)),
        table::column(bold("Value"), |(_, val): (&str, &str)| text_view(val))
            .width(Length::FillPortion(2)),
    ];

    container(scrollable(table(columns, values)))
        .style(|t: &Theme| container::Style {
            border: container::bordered_box(t).border,
            ..container::transparent(t)
        })
        .into()
}
