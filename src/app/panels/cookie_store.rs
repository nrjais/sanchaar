use crate::components::bold;
use cookie_store::Cookie;
use iced::Length;
use iced::widget::text::Wrapping;
use iced::widget::{scrollable, table, text};
use iced::{Element, Task};

use crate::state::tabs::cookies_tab::CookiesTab;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum CookieTabMsg {}

impl CookieTabMsg {
    pub fn update(self, state: &mut AppState) -> Task<Self> {
        let Some(Tab::CookieStore(_tab)) = state.active_tab_mut() else {
            return Task::none();
        };
        match self {}
    }
}

pub fn view<'a>(tab: &'a CookiesTab) -> Element<'a, CookieTabMsg> {
    let columns = [
        table::column(bold("Name"), |cookie: Cookie<'static>| {
            text(cookie.name().to_string())
        })
        .width(Length::FillPortion(1)),
        table::column(bold("Value"), |cookie: Cookie<'static>| {
            text(cookie.value().to_string()).wrapping(Wrapping::Glyph)
        })
        .width(Length::FillPortion(2)),
        table::column(bold("Domain"), |cookie: Cookie<'static>| {
            text(cookie.domain().unwrap_or_default().to_string())
        })
        .width(Length::FillPortion(1)),
        table::column(bold("Path"), |cookie: Cookie<'static>| {
            text(cookie.path().unwrap_or_default().to_string())
        }),
        table::column(bold("Secure"), |cookie: Cookie<'static>| {
            text(
                cookie
                    .secure()
                    .map(|s| if s { "Secure" } else { "Insecure" })
                    .unwrap_or_default()
                    .to_string(),
            )
        }),
    ];

    scrollable(table(columns, tab.cookies())).into()
}
