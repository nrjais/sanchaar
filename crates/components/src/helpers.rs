use iced::{
    widget::{container, text, tooltip::Position, Container, Tooltip},
    Element, Length,
};

pub fn expanded<'a, M>(base: impl Into<Element<'a, M>>) -> Container<'a, M> {
    container(base).width(Length::Fill).height(Length::Fill)
}

pub fn tooltip<'a, M: 'a>(msg: &'static str, base: impl Into<Element<'a, M>>) -> Tooltip<'a, M> {
    iced::widget::tooltip(
        container(text(msg))
            .padding([2, 4])
            .style(container::bordered_box),
        base,
        Position::Bottom,
    )
}
