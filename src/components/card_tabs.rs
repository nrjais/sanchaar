use crate::components::icon;
use iced::widget::{container, horizontal_space};
use iced::{
    theme,
    widget::{button, Row, Text},
    Border, Element, Shadow, Theme,
};
use iced_aw::NerdIcon;

#[derive(Debug, Clone)]
pub enum TabBarAction<T: Clone> {
    NewTab,
    CloseTab(T),
    ChangeTab(T),
}

pub struct CardTab<'a, T> {
    pub id: T,
    pub icon: Text<'a>,
    pub label: Text<'a>,
}

pub fn card_tab<'a, T: Eq>(id: T, icon: Text<'a>, label: Text<'a>) -> CardTab<'a, T> {
    CardTab { id, icon, label }
}

pub fn card_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: Vec<CardTab<'a, T>>,
    on_action: impl Fn(TabBarAction<T>) -> M,
    suffix: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let mut tabs_row = Row::new().align_items(iced::Alignment::Center).spacing(2);
    for tab in tabs {
        let active = tab.id == active;

        let label = Row::new()
            .push(tab.icon)
            .push(tab.label)
            .push(
                button(icon(NerdIcon::CloseBox).size(16))
                    .style(theme::Button::Text)
                    .padding([0, 4])
                    .on_press(on_action(TabBarAction::CloseTab(tab.id.clone()))),
            )
            .align_items(iced::Alignment::Center)
            .spacing(4);

        tabs_row = if active {
            tabs_row.push(
                container(
                    button(label)
                        .style(theme::Button::Positive)
                        .on_press(on_action(TabBarAction::ChangeTab(tab.id.clone()))),
                )
                .padding(1)
                .style(|theme: &Theme| container::Appearance {
                    border: Border {
                        radius: 3.into(),
                        color: theme.extended_palette().background.weak.color,
                        width: 2.0,
                    },
                    ..container::Appearance::default()
                }),
            )
        } else {
            tabs_row.push(
                button(label)
                    .style(theme::Button::Secondary)
                    .on_press(on_action(TabBarAction::ChangeTab(tab.id.clone()))),
            )
        }
    }

    tabs_row = tabs_row.push(
        button(icon(NerdIcon::PlusBox).size(24))
            .style(theme::Button::Text)
            .padding([0, 4])
            .on_press(on_action(TabBarAction::NewTab)),
    );

    if let Some(suffix) = suffix {
        tabs_row = tabs_row.push(horizontal_space()).push(suffix);
    }

    container(tabs_row)
        .width(iced::Length::Fill)
        .style(|theme: &Theme| container::Appearance {
            border: Border {
                radius: 3.into(),
                color: theme.extended_palette().background.weak.color,
                width: 1.0,
            },
            shadow: Shadow {
                color: theme.extended_palette().background.strong.color,
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 2.0,
            },
            ..container::Appearance::default()
        })
        .into()
}
