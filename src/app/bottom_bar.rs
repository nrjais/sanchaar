use crate::components::{NerdIcon, bordered_top, icon, icons, split, tooltip};
use iced::{
    Element, Task,
    widget::{Row, Tooltip, button, space, text},
};
use iced::{border, padding};

use crate::state::{AppState, Tab, UpdateStatus, popups::Popup, tabs::cookies_tab::CookiesTab};

#[derive(Debug, Clone)]
pub enum BottomBarMsg {
    OpenSettings,
    OpenCookies,
    ToggleSplit,
    ToggleSideBar,
    OpenUpdateConfirmation,
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
                state.split_direction = state.split_direction.toggle();
                Task::none()
            }
            OpenUpdateConfirmation => {
                Popup::update_confirmation(&mut state.common);
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

    let split_icon = match state.split_direction {
        split::Direction::Vertical => icons::SplitVertical,
        split::Direction::Horizontal => icons::SplitHorizontal,
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

    let update_status = match state.update_status {
        UpdateStatus::None => None,
        UpdateStatus::Available => Some(("Update available", icons::Download, true)),
        UpdateStatus::Downloading => Some(("Downloading update...", icons::Download, false)),
        UpdateStatus::Installing => Some(("Installing update...", icons::Gear, false)),
        UpdateStatus::Completed => Some(("Update ready - restart app", icons::CheckBold, false)),
    };

    let mut row = Row::new().push(buttons);

    if let Some((status_text, status_icon, clickable)) = update_status {
        let status_content = Row::new()
            .push(icon(status_icon).size(14))
            .push(text(status_text).size(12))
            .spacing(4)
            .align_y(iced::Alignment::Center);

        let status_display: Element<BottomBarMsg> = if clickable {
            button(status_content)
                .on_press(BottomBarMsg::OpenUpdateConfirmation)
                .style(button::text)
                .padding(4)
                .into()
        } else {
            status_content.into()
        };

        row = row.push(space::horizontal()).push(status_display);
    }

    row = row.push(space::horizontal()).spacing(2).padding([0, 4]);
    bordered_top(2, row)
}
