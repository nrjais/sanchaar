use iced::{
    widget::{text, Text},
    Renderer, Theme,
};
use iced_aw::{graphics::icons, NerdIcon};

pub fn icon<'a>(icon: NerdIcon) -> Text<'a, Theme, Renderer> {
    text(icon)
        .shaping(text::Shaping::Advanced)
        .font(icons::NERD_FONT)
}
