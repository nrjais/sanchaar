use iced::{widget::text, Element};
use iced_aw::{graphics::icons, NerdIcon};

pub fn icon<'a, Message>(icon: NerdIcon) -> Element<'a, Message> {
    text(icon).shaping(text::Shaping::Advanced).font(icons::NERD_FONT).into()
}
