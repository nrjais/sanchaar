use crate::components::{bold, icon, icon_button, icons, tooltip};
use cookie_store::Cookie;
use iced::widget::text::Wrapping;
use iced::widget::{button, container, scrollable, table, text};
use iced::{Alignment, Element, Length, Task};

use crate::state::tabs::cookies_tab::CookiesTab;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum CookieTabMsg {
    DeleteCookie(String, String, String),
    ClearAllCookies,
}

impl CookieTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::CookieStore(tab)) = state.active_tab_mut() else {
            return Task::none();
        };
        match self {
            CookieTabMsg::DeleteCookie(name, domain, path) => {
                tab.delete_cookie(&name, &domain, &path);
                Task::none()
            }
            CookieTabMsg::ClearAllCookies => {
                tab.clear_all();
                Task::none()
            }
        }
    }
}

pub fn view<'a>(tab: &'a CookiesTab) -> Element<'a, CookieTabMsg> {
    let columns = [
        table::column(bold("Name"), |cookie: Cookie<'static>| {
            text(cookie.name().to_string())
        })
        .width(Length::FillPortion(1))
        .align_y(Alignment::Center),
        table::column(bold("Value"), |cookie: Cookie<'static>| {
            text(cookie.value().to_string()).wrapping(Wrapping::Glyph)
        })
        .width(Length::FillPortion(2))
        .align_y(Alignment::Center),
        table::column(bold("Domain"), |cookie: Cookie<'static>| {
            text(cookie.domain().unwrap_or_default().to_string())
        })
        .width(Length::FillPortion(1))
        .align_y(Alignment::Center),
        table::column(bold("Path"), |cookie: Cookie<'static>| {
            text(cookie.path().unwrap_or_default().to_string())
        })
        .align_y(Alignment::Center),
        table::column(bold("Secure"), |cookie: Cookie<'static>| {
            text(
                cookie
                    .secure()
                    .map(|s| if s { "Secure" } else { "Insecure" })
                    .unwrap_or_default()
                    .to_string(),
            )
        })
        .align_y(Alignment::Center),
        table::column(text(""), |cookie: Cookie<'static>| {
            let name = cookie.name().to_string();
            let domain = cookie.domain().unwrap_or_default().to_string();
            let path = cookie.path().unwrap_or_default().to_string();

            tooltip(
                "Delete cookie",
                button(icon(icons::Delete).size(16))
                    .padding([0, 4])
                    .style(button::text)
                    .on_press(CookieTabMsg::DeleteCookie(name, domain, path)),
            )
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    ];

    let table_view = scrollable(table(columns, tab.cookies()).padding_x(8).padding_y(4));

    container(table_view).style(container::bordered_box).into()
}
