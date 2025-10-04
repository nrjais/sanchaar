use std::borrow::Cow;
use std::ops::Not;

use iced::widget::{Column, Row, button, container, scrollable, space, text, text_input};
use iced::{Element, Length, Task};

use core::http::CollectionKey;
use core::http::collection::{Collection, Entry, Folder, FolderId};

use crate::commands::builders::save_tab_request_cmd;
use crate::components::{icon, icons};
use crate::state::AppState;
use crate::state::popups::{Popup, SaveRequestState};

#[derive(Debug, Clone)]
pub enum Message {
    Done(CollectionKey),
    NameChanged(String),
    SelectDir(FolderId),
    SelectCollection(CollectionKey),
    ClearSelection,
    ClearDirectory,
    Close,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Task<Message> {
        let common = &mut state.common;
        let Some(Popup::SaveRequest(ref mut data)) = common.popup else {
            return Task::none();
        };

        match self {
            Message::Done(col) => {
                let name = data.name.clone();
                let tab = data.tab;
                let folder = data.folder_id;
                save_tab_request_cmd(state, name, tab, col, folder).map(|_| Message::Close)
            }
            Message::NameChanged(name) => {
                data.name = name;
                Task::none()
            }
            Message::SelectDir(folder) => {
                data.folder_id = Some(folder);
                Task::none()
            }
            Message::ClearSelection => {
                data.col = None;
                data.folder_id = None;
                Task::none()
            }
            Message::ClearDirectory => {
                data.folder_id = None;
                Task::none()
            }
            Message::SelectCollection(col) => {
                if data.col != Some(col) {
                    data.col = Some(col);
                    data.folder_id = None;
                }
                Task::none()
            }
            Message::Close => {
                common.popup = None;
                Task::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Save Request")
}

pub fn done(data: &SaveRequestState) -> Option<Message> {
    if let Some(col) = data.col {
        data.name.is_empty().not().then_some(Message::Done(col))
    } else {
        None
    }
}

pub fn col_selector<'a>(state: &'a AppState) -> Element<'a, Message> {
    let collections = state
        .common
        .collections
        .iter()
        .map(|(k, c)| {
            let name = c.name.as_str();
            button(text(name))
                .on_press(Message::SelectCollection(k))
                .style(button::subtle)
                .width(Length::Fill)
                .padding([2, 4])
                .into()
        })
        .collect();

    Column::from_vec(collections)
        .align_x(iced::Alignment::Center)
        .width(Length::Fill)
        .into()
}

pub fn dir_selector<'a>(
    collection: &'a Collection,
    folder: Option<&'a Folder>,
) -> Element<'a, Message> {
    let children = match folder {
        Some(folder) => &folder.entries,
        _ => &collection.entries,
    };

    let entries: Vec<Element<Message>> = children
        .iter()
        .map(|e| match e {
            Entry::Folder(Folder { id, name, .. }) => button(text(name))
                .padding([2, 4])
                .style(button::subtle)
                .on_press(Message::SelectDir(*id))
                .width(Length::Fill)
                .into(),
            Entry::Item(item) => Row::new()
                .push(icon(icons::API))
                .push(text(&item.name))
                .spacing(8)
                .padding([2, 4])
                .align_y(iced::Alignment::Center)
                .width(Length::Fill)
                .into(),
        })
        .collect();

    Column::from_vec(entries)
        .width(Length::Fill)
        .align_x(iced::Alignment::Center)
        .into()
}

fn breadcrumb<'a>(txt: &'a str, msg: Message) -> Element<'a, Message> {
    button(text(txt))
        .on_press(msg)
        .style(button::text)
        .padding([0, 2])
        .into()
}

pub fn view<'a>(state: &'a AppState, data: &'a SaveRequestState) -> Element<'a, Message> {
    let collection = data.col.and_then(|col| state.common.collections.get(col));

    let name = Row::new()
        .push(text("Name"))
        .push(space::horizontal())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .align_y(iced::Alignment::Center)
        .spacing(4);

    let directory_path = collection
        .zip(data.folder_id)
        .map(|(c, f)| c.folder_path(f))
        .unwrap_or_default();

    let mut path = Row::new()
        .align_y(iced::Alignment::Center)
        .push(breadcrumb("Collection", Message::ClearSelection))
        .push(collection.map(|_| text("/")))
        .push(collection.map(|c| breadcrumb(&c.name, Message::ClearDirectory)));

    for folder in directory_path.iter() {
        path = path.push(text("/"));
        path = path.push(breadcrumb(&folder.name, Message::SelectDir(folder.id)));
    }

    let folder_selector = collection.map(|c| dir_selector(c, directory_path.last().copied()));

    let col_selector = scrollable(folder_selector.unwrap_or(col_selector(state)))
        .width(Length::Fill)
        .spacing(12)
        .height(Length::Fixed(300.0));

    let col_selector = container(col_selector)
        .padding(4)
        .style(container::bordered_box);

    Column::new()
        .push(name)
        .push(container(text("Save to")))
        .push(
            container(path.wrap())
                .style(container::bordered_box)
                .padding(4),
        )
        .push(col_selector)
        .width(400)
        .spacing(8)
        .into()
}
