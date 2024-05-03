use iced::widget::button::Status;
use iced::widget::{horizontal_space, vertical_space};
use iced::Length;
use iced::{
    widget::{button, Column, Row, Text},
    Element,
};

use crate::horizontal_line;
use crate::min_dimension::min_width;

pub struct ButtonTab<'a, T> {
    pub id: T,
    pub label: Box<dyn Fn() -> Text<'a>>,
}

pub fn button_tab<'a, T: Eq>(id: T, label: impl Fn() -> Text<'a> + 'static) -> ButtonTab<'a, T> {
    ButtonTab {
        id,
        label: Box::new(label),
    }
}

pub fn button_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: impl Iterator<Item = ButtonTab<'a, T>>,
    on_tab_change: impl Fn(T) -> M,
    suffix: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let tabs = tab_list(active, tabs, on_tab_change, suffix, false);
    Column::new()
        .push(Row::from_vec(tabs).width(iced::Length::Fill))
        .push(horizontal_line(2))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .align_items(iced::Alignment::Center)
        .into()
}

pub fn vertical_button_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: impl Iterator<Item = ButtonTab<'a, T>>,
    on_tab_change: impl Fn(T) -> M,
) -> Row<'a, M> {
    let tabs = tab_list(active, tabs, on_tab_change, None, true);
    Row::new()
        .push(
            Column::from_vec(tabs)
                .spacing(4)
                .align_items(iced::Alignment::Center),
        )
        .width(iced::Length::Shrink)
        .height(iced::Length::Shrink)
}

fn tab_list<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: impl Iterator<Item = ButtonTab<'a, T>>,
    on_tab_change: impl Fn(T) -> M + Sized,
    suffix: Option<Element<'a, M>>,
    vertical: bool,
) -> Vec<Element<'a, M>> {
    let mut tabs_row = Vec::new();
    for tab in tabs {
        let active = tab.id == active;

        let tab_button = |width: Length| {
            button((tab.label)())
                .style(move |theme, _| {
                    if active {
                        button::secondary(theme, Status::Active)
                    } else {
                        button::text(theme, Status::Active)
                    }
                })
                .width(width)
                .padding([2, 6])
                .on_press(on_tab_change(tab.id.clone()))
        };

        tabs_row.push(if vertical {
            min_width(tab_button(Length::Shrink), tab_button(Length::Fill), 100.).into()
        } else {
            tab_button(Length::Shrink).into()
        });
    }

    if let Some(suffix) = suffix {
        tabs_row.extend([
            vertical
                .then(|| vertical_space())
                .unwrap_or(horizontal_space())
                .into(),
            suffix.into(),
        ]);
    }

    tabs_row
}
