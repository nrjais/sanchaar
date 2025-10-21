use iced::{Element, widget::container};
use iced_core::text;

use crate::widgets::tooltip::{self, Tooltip};

pub fn tooltip<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tooltip: impl Into<Element<'a, Message, Theme, Renderer>>,
    position: tooltip::Position,
) -> Tooltip<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer,
{
    Tooltip::new(content, tooltip, position)
}
