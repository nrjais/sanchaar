use iced::widget::Row;
use iced::Element;

use crate::vertical_line;

pub fn bordered_left<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new().push(vertical_line(width)).push(content).into()
}

pub fn bordered_right<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new().push(content).push(vertical_line(width)).into()
}
