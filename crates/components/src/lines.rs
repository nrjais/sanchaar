use iced::widget::Rule;

pub fn horizontal_line<'a>(width: u16) -> Rule<'a, iced::Theme> {
    Rule::horizontal(width as f32)
}

pub fn vertical_line<'a>(width: u16) -> Rule<'a, iced::Theme> {
    Rule::vertical(width as f32)
}
