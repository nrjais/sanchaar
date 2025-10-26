use iced::{
    Element,
    border::Radius,
    widget::{
        self, Scrollable,
        scrollable::{self, Scrollbar},
    },
};

pub enum Direction {
    Vertical,
    Horizontal,
    Both,
}

pub fn scrollable<'a, Message>(base: impl Into<Element<'a, Message>>) -> Scrollable<'a, Message> {
    scrollable_with(base, Direction::Vertical)
}

pub fn scrollable_with<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    direction: Direction,
) -> Scrollable<'a, Message> {
    let scrollbar: Scrollbar = Scrollbar::default().spacing(0).width(8).scroller_width(8);

    let direction = match direction {
        Direction::Vertical => scrollable::Direction::Vertical(scrollbar),
        Direction::Horizontal => scrollable::Direction::Horizontal(scrollbar),
        Direction::Both => scrollable::Direction::Both {
            vertical: scrollbar,
            horizontal: scrollbar,
        },
    };

    widget::scrollable(base)
        .direction(direction)
        .style(|theme, status| {
            let mut style = scrollable::default(theme, status);
            style.horizontal_rail.scroller.border.radius = Radius::new(100);
            style.vertical_rail.scroller.border.radius = Radius::new(100);

            style
        })
}
