use iced::widget::horizontal_space;
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
    tabs: &[ButtonTab<'a, T>],
    on_tab_change: impl Fn(T) -> M,
    suffix: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let mut tabs_row = Row::new();
    for tab in tabs {
        let active = tab.id == active;

        tabs_row = tabs_row.push(
            button((tab.label)())
                .style(if active {
                    button::primary
                } else {
                    button::text
                })
                .padding([2, 6])
                .on_press(on_tab_change(tab.id.clone())),
        );
    }

    if let Some(suffix) = suffix {
        tabs_row = tabs_row.push(horizontal_space()).push(suffix);
    }

    Column::new()
        .push(tabs_row)
        .push(Rule::horizontal(2.))
        .width(iced::Length::Fill)
        .spacing(2)
        .into()
}
