use crate::components::{NerdIcon, bordered_top, icon, icons, tooltip};
use iced::{
    Element, Task,
    widget::{
        Row, Tooltip, button,
        pane_grid::{self},
        space,
    },
};
use iced::{border, padding};

use crate::state::{AppState, HttpTab, Tab, popups::Popup, tabs::cookies_tab::CookiesTab};

#[derive(Debug, Clone)]
pub enum BottomBarMsg {
    OpenSettings,
    OpenCookies,
    ToggleSplit,
    ToggleSideBar,
}

fn change_axis_for_tabs(state: &mut AppState) {
    for (_, tab) in state.tabs.iter_mut() {
        if let Tab::Http(tab) = tab {
            tab.panes = HttpTab::pane_config(state.split_axis);
        }
    }
}

impl BottomBarMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        use BottomBarMsg::*;

        match self {
            ToggleSideBar => {
                state.pane_config.toggle_side_bar();
                Task::none()
            }
            OpenSettings => {
                Popup::app_settings(&mut state.common);
                Task::none()
            }
            OpenCookies => {
                state.open_unique_tab(Tab::CookieStore(CookiesTab::new(&state.common)));
                Task::none()
            }
            ToggleSplit => {
                state.split_axis = match state.split_axis {
                    pane_grid::Axis::Vertical => pane_grid::Axis::Horizontal,
                    pane_grid::Axis::Horizontal => pane_grid::Axis::Vertical,
                };
                change_axis_for_tabs(state);
                Task::none()
            }
        }
    }
}

fn icon_button<'a>(
    ico: NerdIcon,
    on_press: BottomBarMsg,
    size: Option<u32>,
    desc: &'a str,
) -> Tooltip<'a, BottomBarMsg> {
    let btn = button(icon(ico).size(size.unwrap_or(16)))
        .on_press(on_press)
        .style(|t, s| button::Style {
            border: border::rounded(50),
            ..button::text(t, s)
        })
        .padding(4);

    tooltip(desc, btn)
}

pub fn view(state: &AppState) -> Element<BottomBarMsg> {
    use BottomBarMsg::*;

    let side_bar_icon = if state.pane_config.side_bar_open {
        icons::CloseSideBar
    } else {
        icons::OpenSideBar
    };

    let split_icon = match state.split_axis {
        pane_grid::Axis::Vertical => icons::SplitVertical,
        pane_grid::Axis::Horizontal => icons::SplitHorizontal,
    };

    let buttons = Row::new()
        .push(icon_button(
            side_bar_icon,
            ToggleSideBar,
            Some(12),
            "Toggle Side Panel",
        ))
        .push(icon_button(icons::Gear, OpenSettings, None, "Settings"))
        .push(icon_button(icons::Cookie, OpenCookies, None, "Cookies"))
        .push(icon_button(split_icon, ToggleSplit, None, "Toggle Split"))
        .spacing(16)
        .align_y(iced::Alignment::Center)
        .padding(padding::left(4));

    let row = Row::new()
        .push(buttons)
        .push(space::horizontal())
        .spacing(2)
        .padding([0, 4]);
    bordered_top(2, row)
}
