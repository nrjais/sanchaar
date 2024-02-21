use iced::{
    widget::{button, checkbox, component, container, text_input, Column, Component, Row},
    Border, Element, Theme,
};
use iced_aw::NerdIcon;

use super::icon;

pub struct KeyValViewer<'a, M> {
    values: &'a [(&'a str, &'a str)],
}

impl<'a, M: Clone> KeyValViewer<'a, M> {
    pub fn element(self) -> Element<'a, M>
    where
        M: 'a,
    {
        component(self)
    }
}

pub fn key_value_viewer<M>(values: &KeyValList) -> KeyValViewer<'_, M> {
    KeyValViewer { values }
}

impl<'a, M> Component<M> for KeyValViewer<'a, M> {
    type State = ();

    type Event = ();

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        None
    }

    fn view(&self, state: &Self::State) -> iced::Element<Self::Event> {
        let size = 14;
        let spacing = 2;
        let values = self.values.values().iter().enumerate().map(|(idx, kv)| {
            let name = text(&kv.name)
                .on_input(move |name| KeyValUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(iced::Length::FillPortion(2));
            let value = text(&kv.value)
                .on_input(move |value| KeyValUpdateMsg::ValueChanged(idx, value))
                .size(size)
                .width(iced::Length::FillPortion(3));

            Row::new().push(name).push(value).spacing(spacing).into()
        });

        Column::with_children(values)
            .spacing(spacing)
            .padding([4, 0])
            .into()
    }
}
