use iced::widget::{container, horizontal_space};
use iced::{
    theme,
    widget::{button, Column, Row, Rule, Text},
    Element,
};

pub enum ButtonTabLabel<'a> {
    Text(Text<'a>),
}

pub struct ButtonTab<'a, T, M> {
    pub id: T,
    pub label: ButtonTabLabel<'a>,
    pub body: Element<'a, M>,
}

pub fn button_tab<'a, T: Eq, M>(
    id: T,
    label: ButtonTabLabel<'a>,
    body: Element<'a, M>,
) -> ButtonTab<'a, T, M> {
    ButtonTab { id, label, body }
}

pub fn button_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: Vec<ButtonTab<'a, T, M>>,
    on_tab_change: impl Fn(T) -> M,
    suffix: Option<Element<'a, M>>,
) -> Element<'a, M> {
    let mut tabs_row = Row::new();
    let mut active_tab: Option<Element<'a, M>> = None;
    for tab in tabs {
        let active = tab.id == active;
        if active {
            active_tab = Some(tab.body);
        }
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
        .push(container(active_tab.expect("Invalid active tab id")).height(iced::Length::Fill))
        .width(iced::Length::Fill)
        .spacing(2)
        .into()
}
