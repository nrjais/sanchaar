use components::{table, table_value};
use iced::widget::{text, Column};
use iced::Length;
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
    let cookies = tab
        .cookies()
        .into_iter()
        .map(|cookie| -> [Element<'a, CookieTabMsg>; 5] {
            [
                table_value(cookie.name()),
                table_value(cookie.value()),
                table_value(cookie.domain().unwrap_or_default()),
                table_value(cookie.path().unwrap_or_default()),
                table_value(
                    cookie
                        .secure()
                        .map(|s| s.then_some("Secure").unwrap_or("Insecure"))
                        .unwrap_or_default(),
                ),
            ]
        })
        .collect::<Vec<_>>();

    let headers = [
        table_value("Name"),
        table_value("Value"),
        table_value("Domain"),
        table_value("Path"),
        table_value("Secure"),
    ];
    let content = table(headers, cookies, [2, 3, 2, 1, 1]);

    Column::new()
        .push(text("Cookies"))
        .push(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(8)
        .into()
}
