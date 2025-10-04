use crate::components::{horizontal_line, icon, icons};
use iced::widget::button::{Status, Style};
use iced::widget::{Column, space};
use iced::{
    Border, Center, Element, Length, Shadow, Vector,
    widget::{Row, Text, button, container},
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
    let mut tabs_row = Row::new().align_y(Center).spacing(4);
    for CardTab {
        id,
        icon: tab_icon,
        label,
    } in tabs
    {
        let is_active = id == active;
        let change_id = id.clone();
        let close_id = id.clone();

        let close_button = button(icon(icons::Close).size(18).line_height(1.))
            .padding(4)
            .on_press(on_action(TabBarAction::CloseTab(close_id)))
            .style(move |theme: &iced::Theme, status| {
                let palette = theme.extended_palette();
                let mut style = button::text(theme, Status::Active);
                let text_color = match status {
                    Status::Pressed | Status::Hovered if is_active => palette.primary.strong.text,
                    Status::Pressed | Status::Hovered => palette.primary.strong.color,
                    _ if is_active => palette.primary.strong.text,
                    _ => palette.secondary.base.color,
                };

                style.text_color = text_color;
                style
            });

        let tab_content = Row::new()
            .align_y(Center)
            .spacing(8)
            .push(container(tab_icon).padding([0, 2]))
            .push(label)
            .push(close_button);

        tabs_row = tabs_row.push(
            button(tab_content)
                .padding([2, 4])
                .style(move |theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();

                    let (background, text_color) = match (is_active, status) {
                        (true, Status::Hovered | Status::Pressed) => {
                            (palette.primary.base.color, palette.primary.base.text)
                        }
                        (true, _) => (palette.primary.base.color, palette.primary.base.text),
                        (false, Status::Hovered | Status::Pressed) => (
                            palette.background.neutral.color,
                            palette.background.neutral.text,
                        ),
                        (false, _) => (
                            palette.background.weaker.color,
                            palette.background.weaker.text,
                        ),
                    };

                    let border_color = if is_active {
                        palette.primary.strong.color
                    } else if matches!(status, Status::Hovered | Status::Pressed) {
                        palette.background.strong.color
                    } else {
                        palette.background.stronger.color
                    };

                    let shadow = if is_active {
                        Shadow {
                            color: palette.primary.strong.color.scale_alpha(0.25),
                            offset: Vector::new(0.0, 3.0),
                            blur_radius: 4.0,
                        }
                    } else if matches!(status, Status::Hovered) {
                        Shadow {
                            color: palette.background.strong.color.scale_alpha(0.2),
                            offset: Vector::new(0.0, 2.0),
                            blur_radius: 6.0,
                        }
                    } else {
                        Shadow::default()
                    };

                    button::Style {
                        background: Some(background.into()),
                        text_color,
                        border: Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: border_color,
                        },
                        shadow,
                        ..Style::default()
                    }
                })
                .on_press(on_action(TabBarAction::ChangeTab(change_id))),
        )
    }

    tabs_row = tabs_row
        .push(
            button(icon(icons::Plus).size(20).line_height(1.))
                .padding([2, 6])
                .on_press(on_action(TabBarAction::NewTab))
                .style(|theme: &iced::Theme, status| {
                    let palette = theme.extended_palette();
                    let (background, text_color) = match status {
                        Status::Pressed | Status::Hovered => {
                            (palette.primary.strong.color, palette.primary.strong.text)
                        }
                        _ => (palette.background.weaker.color, palette.primary.base.color),
                    };

                    let border_color = if matches!(status, Status::Hovered | Status::Pressed) {
                        palette.primary.strong.color
                    } else {
                        palette.background.stronger.color
                    };

                    Style {
                        background: Some(background.into()),
                        text_color,
                        border: Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: border_color,
                        },
                        shadow: Shadow {
                            color: palette.primary.strong.color.scale_alpha(0.15),
                            offset: Vector::new(0.0, 2.0),
                            blur_radius: 4.0,
                        },
                        ..Style::default()
                    }
                }),
        )
        .push(space::horizontal());

    if let Some(suffix) = suffix {
        tabs_row = tabs_row.push(suffix);
    }

    Column::new()
        .spacing(4)
        .push(tabs_row)
        .push(horizontal_line(2))
        .width(Length::Fill)
        .into()
}
