use iced::widget::{horizontal_space, vertical_space};
use iced::{
    widget::{button, Column, Row, Rule, Text},
    Element,
};

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
        .push(Rule::horizontal(2.))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .spacing(2)
        .into()
}

pub fn vertical_button_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: impl Iterator<Item = ButtonTab<'a, T>>,
    on_tab_change: impl Fn(T) -> M,
    suffix: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let tabs = tab_list(active, tabs, on_tab_change, suffix, true);
    Row::new()
        .push(Column::from_vec(tabs))
        .push(Rule::vertical(2.))
        .width(iced::Length::Shrink)
        .height(iced::Length::Shrink)
        .spacing(4)
        .into()
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

        tabs_row.push(
            button((tab.label)())
                .style(if active {
                    button::primary
                } else {
                    button::text
                })
                .padding([2, 6])
                .on_press(on_tab_change(tab.id.clone()))
                .into(),
        );
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
