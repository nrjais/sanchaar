use iced::widget::tooltip::Position;
use iced::widget::{column, scrollable, text, tooltip};
use iced::{
    widget::{button, checkbox, component, container, text_input, Component, Row},
    Border, Element, Theme,
};
use iced::{Background, Length};
use std::ops::Not;
use std::path::PathBuf;

use super::{icon, icons};

#[derive(Debug, Default)]
pub struct KeyFile {
    pub disabled: bool,
    pub name: String,
    pub path: Option<PathBuf>,
}

impl KeyFile {
    pub fn new(name: &str, path: Option<PathBuf>, disabled: bool) -> Self {
        Self {
            name: name.to_owned(),
            path,
            disabled,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }
}

#[derive(Debug)]
pub struct KeyFileList {
    list: Vec<KeyFile>,
    pub fixed: bool,
}

impl Default for KeyFileList {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyFileList {
    pub fn new() -> Self {
        Self::from(Vec::new(), false)
    }

    pub fn from(list: Vec<KeyFile>, fixed: bool) -> Self {
        if fixed {
            return KeyFileList { list, fixed };
        }

        let last = &list.last();
        match last {
            Some(last) if last.name.is_empty() => KeyFileList { list, fixed },
            Some(_) | None => {
                let mut list = list;
                list.push(KeyFile::default());
                KeyFileList { list, fixed }
            }
        }
    }

    pub fn update(&mut self, msg: FilePickerUpdateMsg) {
        match msg {
            FilePickerUpdateMsg::Toggled(idx, enabled) => self.list[idx].disabled = !enabled,
            FilePickerUpdateMsg::NameChanged(idx, name) => self.list[idx].name = name,
            FilePickerUpdateMsg::FilePicked(idx, file) => {
                if let Some(file) = file {
                    self.list[idx].path = Some(file);
                }
            }
            FilePickerUpdateMsg::Remove(idx) => {
                self.list.remove(idx);
            }
            _ => todo!(),
        }
        if self.fixed {
            return;
        }

        let last = self.list.last();
        if let Some(last) = last {
            if !last.is_empty() {
                self.list.push(KeyFile::default());
            }
        } else {
            self.list.push(KeyFile::default());
        }
    }

    pub fn values(&self) -> &[KeyFile] {
        &self.list
    }

    pub fn size(&self) -> usize {
        self.list.len()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&KeyFile) -> bool,
    {
        self.list.retain(f);
    }

    pub fn insert(&mut self, key: String) {
        self.list.push(KeyFile {
            name: key,
            disabled: false,
            path: None,
        });
    }

    pub fn remove(&mut self, key: &str) {
        self.list.retain(|kv| kv.name != key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.list.iter().any(|kv| kv.name == key)
    }
}

pub struct MultiFilePicker<'a, M> {
    values: &'a KeyFileList,
    on_change: Option<Box<dyn Fn(FilePickerUpdateMsg) -> M + 'a>>,
    on_file_picker: Box<dyn Fn(usize) -> M + 'a>,
}

impl<'a, M: Clone> MultiFilePicker<'a, M> {
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(FilePickerUpdateMsg) -> M,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

#[derive(Debug, Clone)]
pub enum FilePickerUpdateMsg {
    Toggled(usize, bool),
    NameChanged(usize, String),
    FilePicked(usize, Option<PathBuf>),
    Remove(usize),
    OpenFilePicker(usize),
}

pub fn multi_file_picker<M: Clone>(
    values: &KeyFileList,
    on_file_picker: impl Fn(usize) -> M + 'static,
) -> MultiFilePicker<'_, M> {
    MultiFilePicker {
        values,
        on_change: None,
        on_file_picker: Box::new(on_file_picker),
    }
}

impl<'a, M> Component<M> for MultiFilePicker<'a, M> {
    type State = ();

    type Event = FilePickerUpdateMsg;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
        match event {
            FilePickerUpdateMsg::OpenFilePicker(idx) => {
                return Some((self.on_file_picker)(idx));
            }
            _ => self.on_change.as_ref().map(|f| f(event)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let size = 14;
        let spacing = 2;

        let values = self.values.values().iter().enumerate().map(|(idx, kv)| {
            let border = Border::default();
            let enabled = checkbox("", !kv.disabled)
                .on_toggle(move |enabled| FilePickerUpdateMsg::Toggled(idx, enabled))
                .size(size)
                .spacing(spacing);

            let remove = button(container(icon(icons::Delete).size(size + 4)))
                .padding(0)
                .style(button::text)
                .on_press_maybe(if idx < self.values.values().len() - 1 {
                    Some(FilePickerUpdateMsg::Remove(idx))
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
                .on_input(move |name| FilePickerUpdateMsg::NameChanged(idx, name))
                .on_paste(move |name| FilePickerUpdateMsg::NameChanged(idx, name))
                .size(size)
                .width(Length::FillPortion(2));

            let path = kv
                .path
                .as_ref()
                .and_then(|p| p.to_str())
                .unwrap_or("Select a file");

            let value = container(
                Row::new()
                    .push(tooltip(
                        button(icon(icons::FolderOpen).size(size))
                            .on_press(FilePickerUpdateMsg::OpenFilePicker(idx))
                            .style(button::primary)
                            .padding([2, 6]),
                        tt("File picker"),
                        Position::Bottom,
                    ))
                    // TODO: Ellipsis long file path, show tooltip on hover
                    .push(text(path).size(size))
                    .height(Length::Fill)
                    .spacing(spacing)
                    .align_items(iced::Alignment::Center),
            )
            .width(Length::FillPortion(3));

            container(
                Row::new()
                    .push(name)
                    .push(value)
                    .push_maybe(actions)
                    .height(Length::Shrink)
                    .spacing(spacing),
            )
            .style(container::bordered_box)
            .padding(1)
            .into()
        });

        let header = container(
            Row::new()
                .push(text("Name").size(size).width(Length::FillPortion(2)))
                .push(text("File").size(size).width(Length::FillPortion(3)))
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

impl<'a, M: Clone + 'a> Into<Element<'a, M>> for MultiFilePicker<'a, M> {
    fn into(self) -> Element<'a, M> {
        component(self)
    }
}
