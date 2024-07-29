use iced::widget::{column, text};
use iced::{padding, Background, Length};
use iced::{
    widget::{button, checkbox, component, container, text_input, Component, Row},
    Border, Element, Theme,
};
use std::ops::Not;
use std::path::PathBuf;

use crate::tooltip;

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

    pub fn update(&mut self, msg: FilePickerAction) {
        match msg {
            FilePickerAction::Toggled(idx, enabled) => self.list[idx].disabled = !enabled,
            FilePickerAction::NameChanged(idx, name) => self.list[idx].name = name,
            FilePickerAction::FilePicked(idx, file) => {
                if let Some(file) = file {
                    self.list[idx].path = Some(file);
                }
            }
            FilePickerAction::Remove(idx) => {
                self.list.remove(idx);
            }
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
    on_change: Option<Box<dyn Fn(FilePickerAction) -> M + 'a>>,
    on_file_picker: Box<dyn Fn(usize) -> M + 'a>,
}

impl<'a, M: Clone> MultiFilePicker<'a, M> {
    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(FilePickerAction) -> M,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

#[derive(Debug, Clone)]
pub enum FilePickerAction {
    Toggled(usize, bool),
    NameChanged(usize, String),
    FilePicked(usize, Option<PathBuf>),
    Remove(usize),
}

#[derive(Debug, Clone)]
pub enum FilePickerUpdateMsg {
    Action(FilePickerAction),
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
            FilePickerUpdateMsg::OpenFilePicker(idx) => Some((self.on_file_picker)(idx)),
            FilePickerUpdateMsg::Action(action) => self.on_change.as_ref().map(|f| f(action)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<Self::Event> {
        let size = 14;
        let spacing = 2;

        use FilePickerUpdateMsg::*;
        let values = self.values.values().iter().enumerate().map(|(idx, kv)| {
            let border = Border::default();
            let enabled = checkbox("", !kv.disabled)
                .on_toggle(move |enabled| Action(FilePickerAction::Toggled(idx, enabled)))
                .size(size)
                .spacing(spacing);

            let remove = button(container(icon(icons::Delete).size(size + 4)))
                .padding(0)
                .style(button::text)
                .on_press_maybe(if idx < self.values.values().len() - 1 {
                    Some(Action(FilePickerAction::Remove(idx)))
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

            let name = text_input("", &kv.name)
                .style(input_style)
                .on_input(move |name| Action(FilePickerAction::NameChanged(idx, name)))
                .on_paste(move |name| Action(FilePickerAction::NameChanged(idx, name)))
                .size(size)
                .width(Length::FillPortion(2));

            let path = kv
                .path
                .as_ref()
                .and_then(|p| p.to_str())
                .unwrap_or("Select a file");

            const MAX_LEN: usize = 15;
            let ellipsis = if path.len() > MAX_LEN {
                format!("...{}", &path[path.len() - MAX_LEN..])
            } else {
                path.to_owned()
            };

            let value = container(
                Row::new()
                    .push(tooltip(
                        "File picker",
                        button(icon(icons::FolderOpen).size(size))
                            .on_press(FilePickerUpdateMsg::OpenFilePicker(idx))
                            .style(button::primary)
                            .padding([2, 6]),
                    ))
                    // TODO: Ellipsis long file path, show tooltip on hover
                    .push(tooltip(path, text(ellipsis).size(size)))
                    .height(Length::Fill)
                    .spacing(spacing)
                    .align_y(iced::Alignment::Center),
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
                .width(1)
                .color(t.extended_palette().background.strong.color),
            ..container::transparent(t)
        })
        .into();

        column([header])
            .extend(values)
            .width(Length::Fill)
            .padding(padding::right(8))
            .into()
    }
}

impl<'a, M: Clone + 'a> From<MultiFilePicker<'a, M>> for Element<'a, M> {
    fn from(val: MultiFilePicker<'a, M>) -> Self {
        component(val)
    }
}
