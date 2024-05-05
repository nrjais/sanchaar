use iced::widget::tooltip::Position;
use iced::widget::{column, scrollable, text, tooltip};
use iced::{
    widget::{button, checkbox, component, container, text_input, Component, Row},
    Border, Element, Theme,
};
use iced::{Background, Length};
use std::ops::Not;

use crate::text_editor::{self, line_editor, ContentAction};

use super::{icon, icons};

#[derive(Debug, Default)]
pub struct KeyValue {
    pub disabled: bool,
    name: String,
    value: text_editor::Content,
}

impl KeyValue {
    pub fn new(name: &str, value: &str, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            value: text_editor::Content::with_text(value),
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
        lines == 1 && self.value.line(0).map_or(true, |line| line.is_empty())
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
            KeyValUpdateMsg::ValueChanged(idx, action) => {
                self.list[idx].value.perform(action);
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
            value: text_editor::Content::default(),
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
}

#[derive(Debug, Clone)]
pub enum KeyValUpdateMsg {
    Toggled(usize, bool),
    NameChanged(usize, String),
    ValueChanged(usize, ContentAction),
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

            let tt = |msg: &'static str| {
                container(text(msg))
                    .padding([2, 4])
                    .style(container::bordered_box)
            };

            let actions = self.values.fixed.not().then(|| {
                container(
                    Row::new()
                        .push(tooltip(enabled, tt("Enabled"), Position::Bottom))
                        .push(tooltip(remove, tt("Delete"), Position::Bottom))
                        .align_items(iced::Alignment::Center)
                        .spacing(8),
                )
                .style(container::rounded_box)
                .padding([2, 8])
            });

            let input_style = move |theme: &Theme, status: text_input::Status| text_input::Style {
                border,
                ..text_input::default(theme, status)
            };

            let name = text_input("", &kv.name)
                .style(input_style)
                .on_input(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .on_paste(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(Length::FillPortion(2));

            let value = container(
                line_editor(&kv.value)
                    .style(move |t, s| text_editor::Style {
                        border,
                        ..text_editor::default(t, s)
                    })
                    .on_action(move |a| KeyValUpdateMsg::ValueChanged(idx, a))
                    .size(size),
            )
            .width(Length::FillPortion(3));

            container(
                Row::new()
                    .push(name)
                    .push(value)
                    .push_maybe(actions)
                    .spacing(spacing),
            )
            .style(container::bordered_box)
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
                .with_width(1)
                .with_color(t.extended_palette().background.strong.color),
            ..container::transparent(t)
        })
        .into();

        scrollable(column([header]).extend(values).padding([0, 8, 0, 0])).into()
    }
}

impl<'a, M: Clone + 'a> Into<Element<'a, M>> for KeyValEditor<'a, M> {
    fn into(self) -> Element<'a, M> {
        component(self)
    }
}
