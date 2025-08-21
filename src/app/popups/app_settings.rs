use std::borrow::Cow;

use components::{button_tab, button_tabs};
use iced::widget::{Column, Row, horizontal_space, pick_list, text};
use iced::{Element, Task, Theme};

use crate::state::AppState;
use crate::state::popups::{AppSettingTabs, AppSettingsState, Popup};

#[derive(Debug, Clone)]
pub enum Message {
    TabChange(AppSettingTabs),
    ChangeTheme(Theme),
    Done,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let Some(Popup::AppSettings(data)) = state.common.popup.as_mut() else {
            return Task::none();
        };

        match self {
            Message::Done => {
                state.common.popup = None;
            }
            Message::TabChange(tab) => {
                data.active_tab = tab;
            }
            Message::ChangeTheme(theme) => {
                state.theme = theme;
            }
        }
        Task::none()
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Settings")
}

pub fn done(_data: &AppSettingsState) -> Option<Message> {
    Some(Message::Done)
}

pub(crate) fn view<'a>(state: &'a AppState, data: &'a AppSettingsState) -> Element<'a, Message> {
    let tab_bar = button_tabs(
        data.active_tab,
        [button_tab(AppSettingTabs::General, move || text("General"))].into_iter(),
        Message::TabChange,
        None,
    );
    let content = match data.active_tab {
        AppSettingTabs::General => general_tab(state),
    };

    Column::new()
        .push(tab_bar)
        .push(content)
        .spacing(16)
        .width(400)
        .into()
}

fn general_tab(state: &AppState) -> Element<Message> {
    let size = 14;
    let theme = Row::new()
        .push(text("Theme"))
        .push(horizontal_space())
        .push(pick_list(Theme::ALL, Some(&state.theme), Message::ChangeTheme).text_size(size))
        .align_y(iced::Alignment::Center);

    Column::new().push(theme).spacing(8).into()
}
