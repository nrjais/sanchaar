use crate::{horizontal_line, icon, icons};
use iced::border;
use iced::widget::button::{Status, Style};
use iced::widget::{Column, space};
use iced::{
    Center, Element,
    widget::{Row, Text, button},
};

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
    let mut tabs_row = Row::new().align_y(Center).spacing(2);
    for tab in tabs {
        let active = tab.id == active;

        let label = Row::new()
            .push(tab.icon)
            .push(tab.label)
            .push(
                button(icon(icons::Close).size(20).line_height(1.))
                    .padding([0, 4])
                    .on_press(on_action(TabBarAction::CloseTab(tab.id.clone())))
                    .style(|theme, status| match status {
                        Status::Hovered | Status::Pressed => button::Style {
                            text_color: theme.extended_palette().primary.strong.color,
                            ..button::text(theme, Status::Hovered)
                        },
                        _ => button::text(theme, Status::Active),
                    }),
            )
            .padding([4, 2])
            .align_y(Center)
            .spacing(4);

        tabs_row = tabs_row.push(
            button(label)
                .padding(if active { [2, 4] } else { [0, 4] })
                .style(move |theme, _status| {
                    if active {
                        button::Style {
                            border: border::rounded(border::top(4)),
                            ..button::secondary(theme, Status::Active)
                        }
                    } else {
                        button::secondary(theme, Status::Disabled)
                    }
                })
                .on_press(on_action(TabBarAction::ChangeTab(tab.id.clone()))),
        )
    }

    tabs_row = tabs_row
        .push(
            button(icon(icons::Plus).size(24).line_height(1.))
                .padding([0, 4])
                .on_press(on_action(TabBarAction::NewTab))
                .style(|theme, status| match status {
                    Status::Hovered | Status::Pressed => Style {
                        text_color: theme.extended_palette().primary.strong.color,
                        ..button::text(theme, Status::Active)
                    },
                    _ => button::text(theme, Status::Active),
                }),
        )
        .push(space::horizontal());

    if let Some(suffix) = suffix {
        tabs_row = tabs_row.push(suffix);
    }

    Column::new()
        .push(tabs_row)
        .push(horizontal_line(2))
        .width(iced::Length::Fill)
        .into()
}
