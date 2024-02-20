use iced::{Color, Element};

use crate::components::{button_tab, button_tabs, ButtonTabLabel};
use crate::core::client::Response;
use crate::state::{response::ResponseTabId, AppState};
use iced::theme::Text;
use iced::widget::{text, Row};

#[derive(Debug, Clone)]
pub enum BodyViewerMsg {
    TabChanged(ResponseTabId),
}

impl BodyViewerMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        match self {
            Self::TabChanged(tab) => {
                state.active_tab_mut().response.active_tab = tab;
            }
        }
    }
}

fn fmt_duration(d: std::time::Duration) -> String {
    let millis = d.as_millis();

    let mut duration = String::from("Time:");
    if millis > 1000 {
        duration.push_str(&format!(" {}s", millis / 1000));
    }
    let millis = millis % 1000;
    if millis > 0 {
        duration.push_str(&format!(" {}ms", millis));
    }

    duration
}

fn status_color(status: reqwest::StatusCode) -> Color {
    match status.as_u16() {
        200..=299 => Color::from_rgb8(0, 200, 0),
        300..=399 => Color::from_rgb8(0, 0, 200),
        400..=499 => Color::from_rgb8(200, 200, 0),
        500..=599 => Color::from_rgb8(200, 0, 0),
        _ => Color::WHITE,
    }
}

pub(crate) fn view<'a>(state: &AppState, res: &Response) -> Element<'a, BodyViewerMsg> {
    let active_tab = state.active_tab().response.active_tab;

    let status_size = 12;
    let status = Row::new()
        .push(
            text(res.status.to_string())
                .size(status_size)
                .style(Text::Color(status_color(res.status))),
        )
        .push(
            text(fmt_duration(res.duration))
                .size(status_size)
                .style(Text::Color(Color::from_rgb8(160, 160, 160))),
        )
        .padding([4, 0, 0, 0])
        .spacing(8);

    let tabs = Vec::from([
        button_tab(
            ResponseTabId::Body,
            ButtonTabLabel::Text(text("Body")),
            text("Body").into(),
        ),
        button_tab(
            ResponseTabId::Headers,
            ButtonTabLabel::Text(text("Headers")),
            text("Headers").into(),
        ),
    ]);

    button_tabs(
        active_tab,
        tabs,
        BodyViewerMsg::TabChanged,
        Some(status.into()),
    )
}
