use iced::widget::horizontal_space;
use iced::{
    theme,
    widget::{button, Column, Row, Rule, Text},
    Element,
};

pub enum ButtonTabLabel<'a> {
    Text(Text<'a>),
}

pub struct ButtonTab<'a, T> {
    pub id: T,
    pub label: ButtonTabLabel<'a>,
}

pub fn button_tab<T: Eq>(id: T, label: ButtonTabLabel) -> ButtonTab<T> {
    ButtonTab { id, label }
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
        let ButtonTabLabel::Text(tab_label) = &tab.label;

        tabs_row = tabs_row.push(
            button(tab_label.clone())
                .style(if active {
                    theme::Button::Primary
                } else {
                    theme::Button::Text
                })
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
