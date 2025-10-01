use iced::Element;
use iced::widget::{Column, Row};

use crate::components::{horizontal_line, vertical_line};

pub fn bordered_left<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new().push(vertical_line(width)).push(content).into()
}

pub fn bordered_right<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Row::new().push(content).push(vertical_line(width)).into()
}

pub fn bordered_top<'a, M: 'a>(width: u16, content: impl Into<Element<'a, M>>) -> Element<'a, M> {
    Column::new()
        .push(horizontal_line(width))
        .push(content)
        .into()
}

pub fn bordered_bottom<'a, M: 'a>(
    width: u16,
    content: impl Into<Element<'a, M>>,
) -> Element<'a, M> {
    Column::new()
        .push(content)
        .push(horizontal_line(width))
        .into()
}
