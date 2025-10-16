use crate::components::{NerdIcon, bordered_top, icon, icons, split, tooltip};
use iced::{
    Alignment, Element, Task,
    widget::{Row, Tooltip, button, space, text},
};
use iced::{border, padding};

use crate::state::{AppState, Tab, UpdateStatus, popups::Popup, tabs::cookies_tab::CookiesTab};
use iced_auto_updater_plugin::ReleaseInfo;

#[derive(Debug, Clone)]
pub enum BottomBarMsg {
    OpenSettings,
    OpenCookies,
    ToggleSplit,
    ToggleSideBar,
    OpenUpdateConfirmation(ReleaseInfo),
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
                state.split_direction = state.split_direction.opposite();
                Task::none()
            }
            OpenUpdateConfirmation(release) => {
                Popup::update_confirmation(&mut state.common, release);
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
        .align_y(Alignment::Center);

    let update_status = match state.update_status {
        UpdateStatus::None => None,
        UpdateStatus::Available(ref release) => Some((None, icons::Download, Some(release))),
        UpdateStatus::Downloading => Some((Some("Downloading"), icons::DotsCircle, None)),
        UpdateStatus::Installing => Some((Some("Installing"), icons::DotsCircle, None)),
        UpdateStatus::Completed => Some((Some("Restart to apply"), icons::Replay, None)),
    };

    let mut row = Row::new()
        .push(buttons)
        .padding(padding::left(8).right(12))
        .push(space::horizontal())
        .align_y(Alignment::Center);

    if let Some((status_text, status_icon, release)) = update_status {
        let status_text = status_text.map(|s| text(s).size(12));
        let status_content = Row::new()
            .push(icon(status_icon).size(16))
            .push(status_text)
            .spacing(8)
            .align_y(Alignment::Center);

        let status_display: Element<BottomBarMsg> = if let Some(release) = release {
            let btn = button(status_content)
                .on_press(BottomBarMsg::OpenUpdateConfirmation(release.clone()))
                .style(|t, s| button::Style {
                    border: border::rounded(50),
                    ..button::text(t, s)
                })
                .padding(4);

            tooltip("Update available", btn).into()
        } else {
            status_content.into()
        };

        row = row.push(status_display);
    }

    bordered_top(2, row)
}
