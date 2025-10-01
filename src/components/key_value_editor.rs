use iced::widget::{column, text};
use iced::{Background, Length, Padding, border, padding};
use iced::{
    Border, Element, Theme,
    widget::{Row, button, checkbox, container, text_input},
};
use std::collections::HashSet;
use std::ops::Not;
use std::sync::Arc;

use crate::components::editor;
use crate::components::{LineEditorMsg, line_editor, tooltip};

use super::{icon, icons};

#[derive(Debug, Default)]
pub struct KeyValue {
    pub disabled: bool,
    name: String,
    value: editor::Content,
}

impl KeyValue {
    pub fn new(name: &str, value: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            value: editor::Content::with_text(value),
            disabled,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> String {
        self.value.text()
    }

    pub fn is_value_empty(&self) -> bool {
        let lines = self.value.line_count();
        lines == 1 && self.value.line(0).is_none_or(|line| line.text.is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.is_value_empty()
    }
}

#[derive(Debug)]
pub struct KeyValList {
    list: Vec<KeyValue>,
    pub fixed: bool,
}

impl Default for KeyValList {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyValList {
    pub fn new() -> Self {
        Self::from(Vec::new(), false)
    }

    pub fn from(list: Vec<KeyValue>, fixed: bool) -> Self {
        if fixed {
            return KeyValList { list, fixed };
        }

        let last = &list.last();
        match last {
            Some(last) if last.name.is_empty() => KeyValList { list, fixed },
            Some(_) | None => {
                let mut list = list;
                list.push(KeyValue::default());
                KeyValList { list, fixed }
            }
        }
    }

    pub fn update(&mut self, msg: KeyValUpdateMsg) {
        match msg {
            KeyValUpdateMsg::Toggled(idx, enabled) => self.list[idx].disabled = !enabled,
            KeyValUpdateMsg::NameChanged(idx, name) => self.list[idx].name = name,
            KeyValUpdateMsg::ValueChanged(idx, msg) => {
                msg.update(&mut self.list[idx].value);
            }
            KeyValUpdateMsg::Remove(idx) => {
                self.list.remove(idx);
            }
        }
        if self.fixed {
            return;
        }

        let last = self.list.last();
        if let Some(last) = last {
            if !last.is_empty() {
                self.list.push(KeyValue::default());
            }
        } else {
            self.list.push(KeyValue::default());
        }
    }

    pub fn values(&self) -> &[KeyValue] {
        &self.list
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&KeyValue) -> bool,
    {
        self.list.retain(f);
    }

    pub fn insert(&mut self, key: String) {
        self.list.push(KeyValue {
            name: key,
            disabled: false,
            value: editor::Content::default(),
        });
    }

    pub fn remove(&mut self, key: &str) {
        self.list.retain(|kv| kv.name != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.list.iter().any(|kv| kv.name == key)
    }
}

pub struct KeyValEditor<'a> {
    values: &'a KeyValList,
    padding: Padding,
    vars: Arc<HashSet<String>>,
}

impl<'a> KeyValEditor<'a> {
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn vars(mut self, vars: Arc<HashSet<String>>) -> Self {
        self.vars = vars;
        self
    }

    pub fn on_change<M: Clone + 'a>(self, f: impl Fn(KeyValUpdateMsg) -> M + 'a) -> Element<'a, M> {
        self.view().map(f)
    }

    fn view(self) -> Element<'a, KeyValUpdateMsg> {
        let size = 14;
        let spacing = 2;

        let values = &self.values.values();
        let last_idx = values.len() - 1;
        let values = values.iter().enumerate().map(|(idx, kv)| {
            let border = Border::default();
            let enabled = checkbox("", !kv.disabled)
                .on_toggle(move |enabled| KeyValUpdateMsg::Toggled(idx, enabled))
                .size(size)
                .spacing(spacing);

            let remove = button(container(icon(icons::Delete).size(size + 4)))
                .padding(0)
                .style(button::text)
                .on_press_maybe(if idx < self.values.values().len() - 1 {
                    Some(KeyValUpdateMsg::Remove(idx))
                } else {
                    None
                });

            let actions = self.values.fixed.not().then(|| {
                container(
                    Row::new()
                        .push(tooltip("Enabled", enabled))
                        .push(tooltip("Delete", remove))
                        .align_y(iced::Alignment::Center)
                        .spacing(8),
                )
                .style(container::rounded_box)
                .padding([2, 8])
            });

            let input_style = move |theme: &Theme, status: text_input::Status| text_input::Style {
                border,
                ..text_input::default(theme, status)
            };

            let name = text_input("Name", &kv.name)
                .style(input_style)
                .on_input(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .on_paste(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(Length::FillPortion(2));

            let value = container(
                line_editor(&kv.value)
                    .placeholder("Value")
                    .style(move |t, s| editor::Style {
                        border,
                        ..editor::default(t, s)
                    })
                    .vars(Arc::clone(&self.vars))
                    .size(size)
                    .map(move |a| KeyValUpdateMsg::ValueChanged(idx, a)),
            )
            .width(Length::FillPortion(3));

            container(
                Row::new()
                    .push(name)
                    .push(value)
                    .push(actions)
                    .spacing(spacing),
            )
            .style(move |t| {
                let last = idx == last_idx;
                let style = container::bordered_box(t);

                let radius = if last {
                    border::bottom(2)
                } else {
                    border::radius(0)
                };

                container::Style {
                    border: style.border.rounded(radius),
                    ..style
                }
            })
            .padding(1)
            .into()
        });

        let header = container(
            Row::new()
                .push(text("Name").size(size).width(Length::FillPortion(2)))
                .push(text("Value").size(size).width(Length::FillPortion(3)))
                .push(text("Actions").size(size).width(Length::Shrink))
                .spacing(4)
                .padding([2, 4]),
        )
        .style(|t: &Theme| container::Style {
            background: Some(Background::Color(
                t.extended_palette().background.weak.color,
            )),
            border: Border::default()
                .width(1)
                .color(t.extended_palette().background.strong.color)
                .rounded(border::top(2)),
            ..container::transparent(t)
        })
        .into();

        column([header])
            .extend(values)
            .padding(self.padding)
            .width(Length::Fill)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum KeyValUpdateMsg {
    Toggled(usize, bool),
    NameChanged(usize, String),
    ValueChanged(usize, LineEditorMsg),
    Remove(usize),
}

pub fn key_value_editor<'a>(
    values: &'a KeyValList,
    vars: &Arc<HashSet<String>>,
) -> KeyValEditor<'a> {
    KeyValEditor {
        values,
        padding: padding::right(8),
        vars: Arc::clone(vars),
    }
}
