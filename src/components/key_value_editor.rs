use iced::widget::scrollable;
use iced::{
    widget::{button, checkbox, component, container, text_input, Column, Component, Row},
    Border, Element, Theme,
};
use std::ops::Not;

use super::{icon, icons};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyValue {
    pub disabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
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
        KeyValList {
            list: vec![KeyValue::default()],
            fixed: false,
        }
    }

    pub fn empty() -> Self {
        KeyValList {
            list: vec![],
            fixed: true,
        }
    }

    pub fn from(values: Vec<KeyValue>, fixed: bool) -> Self {
        let last = &values.last();
        match last {
            Some(last) if !last.name.is_empty() && !fixed => {
                let mut values = values;
                values.push(KeyValue::default());
                KeyValList {
                    list: values,
                    fixed,
                }
            }
            Some(_) | None => KeyValList {
                list: values,
                fixed,
            },
        }
    }

    pub fn update(&mut self, msg: KeyValUpdateMsg) {
        match msg {
            KeyValUpdateMsg::Toggled(idx, enabled) => self.list[idx].disabled = !enabled,
            KeyValUpdateMsg::NameChanged(idx, name) => self.list[idx].name = name,
            KeyValUpdateMsg::ValueChanged(idx, value) => self.list[idx].value = value,
            KeyValUpdateMsg::Remove(idx) => {
                self.list.remove(idx);
            }
        }
        if self.fixed {
            return;
        }

        let last = self.list.last();
        if let Some(last) = last {
            if !last.name.is_empty() || !last.value.is_empty() {
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
            value: String::new(),
        });
    }

    pub fn remove(&mut self, key: &str) {
        self.list.retain(|kv| kv.name != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.list.iter().any(|kv| kv.name == key)
    }
}

pub struct KeyValEditor<'a, M> {
    values: &'a KeyValList,
    on_change: Option<Box<dyn Fn(KeyValUpdateMsg) -> M + 'a>>,
}

impl<'a, M: Clone> KeyValEditor<'a, M> {
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(KeyValUpdateMsg) -> M,
    {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn element(self) -> Element<'a, M>
    where
        M: 'a,
    {
        component(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyValUpdateMsg {
    Toggled(usize, bool),
    NameChanged(usize, String),
    ValueChanged(usize, String),
    Remove(usize),
}

pub fn key_value_editor<M>(values: &KeyValList) -> KeyValEditor<'_, M> {
    KeyValEditor {
        values,
        on_change: None,
    }
}

impl<'a, M> Component<M> for KeyValEditor<'a, M> {
    type State = ();

    type Event = KeyValUpdateMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        self.on_change.as_ref().map(|f| f(event))
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let size = 14;
        let spacing = 2;

        let values = self.values.values().iter().enumerate().map(|(idx, kv)| {
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
                        .push(enabled)
                        .push(remove)
                        .align_items(iced::Alignment::Center)
                        .spacing(8),
                )
                .padding([2, 8])
                .style(|theme: &Theme| container::Style {
                    border: Border {
                        color: theme.extended_palette().secondary.strong.color,
                        width: 1.,
                        radius: 2.into(),
                    },
                    ..container::Style::default()
                })
            });

            let name = text_input("Key", &kv.name)
                .on_input(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(iced::Length::FillPortion(2));
            let value = text_input("Value", &kv.value)
                .on_input(move |value| KeyValUpdateMsg::ValueChanged(idx, value))
                .size(size)
                .width(iced::Length::FillPortion(3));

            Row::new()
                .push(name)
                .push(value)
                .push_maybe(actions)
                .spacing(spacing)
                .into()
        });

        scrollable(
            Column::with_children(values)
                .spacing(spacing)
                .padding([0, 8, 0, 0]),
        )
        .into()
    }
}
