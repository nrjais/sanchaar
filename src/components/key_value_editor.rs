use iced::{
    widget::{button, checkbox, component, container, text_input, Column, Component, Row},
    Border, Element, Theme,
};
use iced_aw::NerdIcon;

use super::icon;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyValue {
    pub disabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyValList(Vec<KeyValue>);
impl Default for KeyValList {
    fn default() -> Self {
        KeyValList(vec![KeyValue::default()])
    }
}

impl KeyValList {
    pub fn update(&mut self, msg: KeyValUpdateMsg) {
        match msg {
            KeyValUpdateMsg::AddHeader => self.0.push(KeyValue::default()),
            KeyValUpdateMsg::Toggled(idx, enabled) => self.0[idx].disabled = !enabled,
            KeyValUpdateMsg::NameChanged(idx, name) => self.0[idx].name = name,
            KeyValUpdateMsg::ValueChanged(idx, value) => self.0[idx].value = value,
            KeyValUpdateMsg::Remove(idx) => {
                self.0.remove(idx);
            }
        }
        let last = self.0.last();
        if let Some(last) = last {
            if !last.name.is_empty() || !last.value.is_empty() {
                self.0.push(KeyValue::default());
            }
        } else {
            self.0.push(KeyValue::default());
        }
    }

    pub fn values(&self) -> &[KeyValue] {
        &self.0
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
    AddHeader,
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

            let enabled =
                container(enabled)
                    .padding([4, 2])
                    .style(|theme: &Theme| container::Appearance {
                        border: Border {
                            color: theme.extended_palette().secondary.strong.color,
                            width: 1.,
                            radius: 2.into(),
                        },
                        ..container::Appearance::default()
                    });

            let name = text_input("Key", &kv.name)
                .on_input(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(iced::Length::FillPortion(2));
            let value = text_input("Value", &kv.value)
                .on_input(move |value| KeyValUpdateMsg::ValueChanged(idx, value))
                .size(size)
                .width(iced::Length::FillPortion(3));

            let remove = button(container(icon(NerdIcon::TrashCan).size(size)).padding([0, 4]))
                .on_press_maybe(if idx < self.values.values().len() - 1 {
                    Some(KeyValUpdateMsg::Remove(idx))
                } else {
                    None
                });

            Row::new()
                .push(enabled)
                .push(name)
                .push(value)
                .push(remove)
                .spacing(spacing)
                .into()
        });

        Column::with_children(values)
            .spacing(spacing)
            .padding([4, 0])
            .into()
    }
}
