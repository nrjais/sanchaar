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
    use super::NerdIcon;

    pub const TriangleRight: NerdIcon = NerdIcon('');
    pub const TriangleDown: NerdIcon = NerdIcon('');
    pub const CloseBox: NerdIcon = NerdIcon('󰅗');
    pub const PlusBox: NerdIcon = NerdIcon('󰐖');
    pub const Delete: NerdIcon = NerdIcon('󰆴');
    pub const CheckBold: NerdIcon = NerdIcon('󰸞');
    pub const Pencil: NerdIcon = NerdIcon('󰏫');
    pub const FileCancel: NerdIcon = NerdIcon('󰷆');
    pub const Error: NerdIcon = NerdIcon('');
    pub const Send: NerdIcon = NerdIcon('󰒊');
    pub const SendUp: NerdIcon = NerdIcon('');
    pub const ContentSave: NerdIcon = NerdIcon('󰆓');
    pub const DotsCircle: NerdIcon = NerdIcon('󱥸');
    pub const Close: NerdIcon = NerdIcon('');
}
