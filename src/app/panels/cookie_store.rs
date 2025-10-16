use crate::components::{LineEditorMsg, bold, icon, icon_button, icons, line_editor, tooltip};
use cookie_store::Cookie;
use iced::widget::text::Wrapping;
use iced::widget::{button, column, container, row, scrollable, table, text};
use iced::{Alignment, Element, Length, Task};

use crate::state::tabs::cookies_tab::CookiesTab;
use crate::state::{AppState, Tab};

#[derive(Debug, Clone)]
pub enum CookieTabMsg {
    DeleteCookie(String, String, String),
    ClearAllCookies,
    SearchChanged(LineEditorMsg),
    ClearSearch,
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
            CookieTabMsg::SearchChanged(update) => {
                update.update(&mut tab.search_query);
                let query = tab.search_query.text().trim().to_string();
                tab.set_search_query(&query);
                Task::none()
            }
            CookieTabMsg::ClearSearch => {
                tab.clear_search_query();
                Task::none()
            }
        }
    }
}

pub fn view<'a>(tab: &'a CookiesTab) -> Element<'a, CookieTabMsg> {
    let is_empty = tab.search_query_text.is_empty();
    let cookies = tab.cookies();

    let search_placeholder = "Search (name, value, domain)...";

    let search_input = container(
        line_editor(&tab.search_query)
            .placeholder(search_placeholder)
            .highlight(false)
            .map(CookieTabMsg::SearchChanged),
    )
    .width(Length::FillPortion(1));

    let clear_all_button = icon_button(icons::Delete, Some(24), Some(8))
        .style(button::danger)
        .on_press_maybe((!cookies.is_empty()).then_some(CookieTabMsg::ClearAllCookies));

    let clear_all_button = tooltip("Remove all cookies", clear_all_button);

    let search_row = row![search_input, clear_all_button]
        .align_y(Alignment::Center)
        .spacing(8);

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
                button(icon(icons::Delete).size(20))
                    .padding([0, 4])
                    .style(button::text)
                    .on_press(CookieTabMsg::DeleteCookie(name, domain, path)),
            )
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center),
    ];

    let content: Element<'a, CookieTabMsg> = if cookies.is_empty() {
        let message = if is_empty {
            "No cookies found"
        } else {
            "No matching cookies found"
        };
        text(message).into()
    } else {
        container(scrollable(
            table(columns, cookies.to_vec()).padding_x(8).padding_y(4),
        ))
        .style(container::bordered_box)
        .into()
    };

    column![search_row, content]
        .spacing(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
