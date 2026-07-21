use std::fmt::Display;

use iced::{
    Alignment::Center,
    Font, Length, Renderer, Theme,
    widget::{Button, Text, button, container, text},
};

pub fn icon<'a>(icon: NerdIcon) -> Text<'a, Theme, Renderer> {
    text(icon.0)
        .align_x(Center)
        .align_y(Center)
        .font(Font::with_family("Hack Nerd Font"))
}

pub fn icon_button<'a, M: 'a>(
    ico: NerdIcon,
    size: Option<u32>,
    padding: Option<u16>,
) -> Button<'a, M> {
    let ico = match size {
        Some(size) => icon(ico).size(size),
        None => icon(ico),
    };

    button(container(ico).padding(padding.map(|h| [0, h]).unwrap_or([0, 0])))
        .padding(0)
        .width(Length::Shrink)
}

pub struct NerdIcon(pub char);

impl Display for NerdIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(dead_code, non_upper_case_globals)]
pub mod icons {
    use super::NerdIcon;

    pub const TriangleRight: NerdIcon = NerdIcon('');
    pub const Cookie: NerdIcon = NerdIcon('󰆘');
    pub const TriangleDown: NerdIcon = NerdIcon('');
    pub const CloseBox: NerdIcon = NerdIcon('󰅗');
    pub const PlusBox: NerdIcon = NerdIcon('󰐖');
    pub const Plus: NerdIcon = NerdIcon('󰐕');
    pub const Delete: NerdIcon = NerdIcon('󰆴');
    pub const CheckBold: NerdIcon = NerdIcon('󰸞');
    pub const Wand: NerdIcon = NerdIcon('');
    pub const Pencil: NerdIcon = NerdIcon('󰏫');
    pub const Gear: NerdIcon = NerdIcon('󰒓');
    pub const Import: NerdIcon = NerdIcon('󰋺');
    pub const Download: NerdIcon = NerdIcon('');
    pub const Copy: NerdIcon = NerdIcon('');
    pub const Filter: NerdIcon = NerdIcon('');
    pub const FileCancel: NerdIcon = NerdIcon('󰷆');
    pub const Error: NerdIcon = NerdIcon('');
    pub const Edit: NerdIcon = NerdIcon('󰷉');
    pub const EditLines: NerdIcon = NerdIcon('󱩽');
    pub const Send: NerdIcon = NerdIcon('󰒊');
    pub const Replay: NerdIcon = NerdIcon('󰑙');
    pub const SendUp: NerdIcon = NerdIcon('');
    pub const Path: NerdIcon = NerdIcon('');
    pub const ContentSave: NerdIcon = NerdIcon('󰆓');
    pub const DotsCircle: NerdIcon = NerdIcon('󱥸');
    pub const Dot: NerdIcon = NerdIcon('•');
    pub const Close: NerdIcon = NerdIcon('󰅖');
    pub const API: NerdIcon = NerdIcon('󰖟');
    pub const Folder: NerdIcon = NerdIcon('󰉋');
    pub const FolderOpen: NerdIcon = NerdIcon('󰝰');
    pub const History: NerdIcon = NerdIcon('󰋚');
    pub const SplitVertical: NerdIcon = NerdIcon('');
    pub const SplitHorizontal: NerdIcon = NerdIcon('');
    pub const OpenSideBar: NerdIcon = NerdIcon('');
    pub const CloseSideBar: NerdIcon = NerdIcon('');
    pub const Speedometer: NerdIcon = NerdIcon('󰓅');
}
