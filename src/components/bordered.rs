use iced::widget::rule::{FillMode, Style};
use iced::widget::{vertical_rule, Row};
use iced::{Element, Theme};

pub fn style<'a>(width: u16) -> impl Fn(&Theme) -> Style + 'a {
    move |theme: &Theme| {
        let palette = theme.extended_palette();

        Style {
            color: palette.background.strong.color,
            width,
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
        }
    }
}

pub fn bordered_left<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new()
        .push(vertical_rule(width).style(style(width)))
        .push(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

pub fn bordered_right<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new()
        .push(content)
        .push(vertical_rule(width).style(style(width)))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}
