use iced::widget::{rule, Rule};

pub fn horizontal_line<'a>(width: u16) -> Rule<'a, iced::Theme> {
    Rule::horizontal(width as f32).style(move |t| rule::Style {
        width,
        ..rule::default(t)
    })
}
