use components::{bordered_left, bordered_right};
use iced::padding;
use iced::widget::pane_grid::ResizeEvent;
use iced::widget::{pane_grid, Column, PaneGrid};
use iced::{widget::container, Element, Task};

use crate::state::{AppState, HttpTab, SplitState, Tab};

const BORDER_WIDTH: u16 = 1;

#[derive(Debug, Clone)]
pub enum CollectionTabMsg {}

impl CollectionTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        match self {}
    }
}

pub fn view<'a>(state: &'a AppState, tab: &'a HttpTab) -> Element<'a, CollectionTabMsg> {
    container("").into()
}
