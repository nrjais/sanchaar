use components::{NerdIcon, bordered_top, icon, icons, tooltip};
use iced::{
    Element, Task,
    widget::{Row, Tooltip, button, space},
};
use iced::{border, padding};

use crate::state::{AppState, Tab, popups::Popup, tabs::cookies_tab::CookiesTab};

#[derive(Debug, Clone)]
pub enum BottomBarMsg {
    OpenSettings,
    OpenCookies,
}

impl BottomBarMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        use BottomBarMsg::*;

        match self {
            OpenSettings => {
                Popup::app_settings(&mut state.common);
                Task::none()
            }
            OpenCookies => {
                state.open_unique_tab(Tab::CookieStore(CookiesTab::new(&state.common)));
                Task::none()
            }
        }
    }
}

fn icon_button<'a>(
    ico: NerdIcon,
    on_press: BottomBarMsg,
    desc: &'static str,
) -> Tooltip<'a, BottomBarMsg> {
    let btn = button(icon(ico).size(16))
        .on_press(on_press)
        .style(|t, s| button::Style {
            border: border::rounded(50),
            ..button::text(t, s)
        })
        .padding(0);

    tooltip(desc, btn)
}

pub fn view(_state: &AppState) -> Element<BottomBarMsg> {
    use BottomBarMsg::*;

    let buttons = Row::new()
        .push(icon_button(icons::Gear, OpenSettings, "Settings"))
        .push(icon_button(icons::Cookie, OpenCookies, "Cookies"))
        .spacing(8)
        .padding(padding::left(4));

    let row = Row::new()
        .push(buttons)
        .push(space::horizontal())
        .spacing(2)
        .padding([0, 4]);
    bordered_top(2, row)
}
