use iced::{Color, Element};

use iced::theme::Text;
use iced::widget::{text, Column, Row};

use crate::components::{
    button_tab, button_tabs, code_viewer, key_value_viewer, ButtonTabLabel, CodeViewerMsg,
    ContentType,
};
use crate::state::response::{CompletedResponse, ResponseState};
use crate::state::{response::ResponseTabId, AppState};

#[derive(Debug, Clone)]
pub enum CompletedMsg {
    TabChanged(ResponseTabId),
    CodeViewerMsg(CodeViewerMsg),
}

impl CompletedMsg {
    pub(crate) fn update(self, state: &mut AppState) {
        let active_tab = state.active_tab_mut();
        match self {
            Self::TabChanged(tab) => {
                active_tab.response.active_tab = tab;
            }
            Self::CodeViewerMsg(msg) => {
                if let ResponseState::Completed(ref mut res) = active_tab.response.state {
                    msg.update(&mut res.content);
                }
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

pub(crate) fn view<'a>(
    state: &'a AppState,
    cr: &'a CompletedResponse,
) -> Element<'a, CompletedMsg> {
    let active_tab = state.active_tab();
    let res = &cr.result;

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

    let headers = res
        .headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or_default()))
        .collect::<Vec<_>>();

    let tab_content = match active_tab.response.active_tab {
        ResponseTabId::Body => code_viewer(&cr.content, ContentType::Json)
            .on_action(CompletedMsg::CodeViewerMsg)
            .element(),
        ResponseTabId::Headers => key_value_viewer(&headers),
    };

    let tabs = button_tabs(
        active_tab.response.active_tab,
        &[
            button_tab(ResponseTabId::Body, ButtonTabLabel::Text(text("Body"))),
            button_tab(
                ResponseTabId::Headers,
                ButtonTabLabel::Text(text("Headers")),
            ),
        ],
        CompletedMsg::TabChanged,
        Some(status.into()),
    );

    Column::new()
        .push(tabs)
        .push(tab_content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .spacing(2)
        .into()
}
