use iced::{
    widget::{text, Text},
    Renderer, Theme,
};

pub fn icon<'a>(icon: NerdIcon) -> Text<'a, Theme, Renderer> {
    text(icon.0).shaping(text::Shaping::Advanced)
}

pub struct NerdIcon(char);

#[allow(dead_code, non_upper_case_globals)]
pub mod icons {
    use crate::components::icon::NerdIcon;

    pub const TriangleRight: NerdIcon = NerdIcon('');
    pub const TriangleDown: NerdIcon = NerdIcon('');
    pub const CloseBox: NerdIcon = NerdIcon('󰅗');
    pub const PlusBox: NerdIcon = NerdIcon('󰐖');
    pub const TrashCan: NerdIcon = NerdIcon('󰩹');
    pub const CheckBold: NerdIcon = NerdIcon('󰸞');
    pub const PencilOutline: NerdIcon = NerdIcon('󰲶');
    pub const FileCancel: NerdIcon = NerdIcon('󰷆');
    pub const Error: NerdIcon = NerdIcon('');
    pub const Send: NerdIcon = NerdIcon('󰒊');
    pub const SendUp: NerdIcon = NerdIcon('');
    pub const ContentSave: NerdIcon = NerdIcon('󰆓');
}
