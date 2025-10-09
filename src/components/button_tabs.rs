use crate::components::min_dimension::min_width;
use iced::Alignment::Center;
use iced::widget::button::Status;
use iced::widget::{container, space};
use iced::{Background, Length};
use iced::{
    Element,
    widget::{Column, Row, Text, button},
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
        .push(Row::from_vec(tabs).spacing(2).width(iced::Length::Fill))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .align_x(Center)
        .into()
}

pub fn vertical_button_tabs<'a, T: Eq + Clone, M: 'a + Clone>(
    active: T,
    tabs: impl Iterator<Item = ButtonTab<'a, T>>,
    on_tab_change: impl Fn(T) -> M,
) -> Row<'a, M> {
    let tabs = tab_list(active, tabs, on_tab_change, None, true);
    Row::new()
        .push(Column::from_vec(tabs).spacing(4).align_x(Center))
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
            let btn = button((tab.label)())
                .style(move |theme, _| button::text(theme, Status::Hovered))
                .width(width)
                .padding([2, 6])
                .on_press(on_tab_change(tab.id.clone()));

            if active {
                Column::new()
                    .push(btn.style(|theme, _| {
                        let palette = theme.extended_palette();
                        let mut style = button::text(theme, Status::Active);
                        style.background = None;
                        style.text_color = palette.background.strong.text;
                        style
                    }))
                    .push(
                        container(space::horizontal())
                            .width(iced::Length::Fill)
                            .height(2.0)
                            .style(move |theme: &iced::Theme| {
                                let palette = theme.extended_palette();
                                container::Style {
                                    background: Some(Background::Color(
                                        palette.primary.strong.color,
                                    )),
                                    ..Default::default()
                                }
                            }),
                    )
                    .width(width)
            } else {
                Column::new()
                    .push(btn)
                    .push(space::vertical().height(2.0))
                    .width(width)
            }
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
                .then(space::vertical)
                .unwrap_or(space::horizontal())
                .into(),
            suffix,
        ]);
    }

    tabs_row
}
