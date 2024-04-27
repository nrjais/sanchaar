use std::borrow::Cow;

use iced::{Command, Element, Length};
use iced::widget::{
    button, Column, container, horizontal_space, Row, scrollable, text, text_input,
};

use core::http::collection::{Collection, Entry, Folder, FolderId};
use core::http::CollectionKey;

use crate::commands::builders::save_new_request;
use crate::state::AppState;
use crate::state::popups::{Popup, SaveRequestState};

#[derive(Debug, Clone)]
pub enum Message {
    Done(CollectionKey),
    NameChanged(String),
    SelectDir(FolderId),
    SelectCollection(CollectionKey),
    Close,
}

impl Message {
    pub fn update(self, state: &mut AppState) -> Command<Message> {
        let Some(Popup::SaveRequest(ref mut data)) = state.popup else {
            return Command::none();
        };
        match self {
            Message::Done(col) => {
                let name = data.name.clone();
                let tab = data.tab;
                let folder = data.folder_id;
                save_new_request(state, name, tab, col, folder, |_| Message::Close)
            }
            Message::NameChanged(name) => {
                data.name = name;
                Command::none()
            }
            Message::SelectDir(folder) => {
                data.folder_id = Some(folder);
                Command::none()
            }
            Message::SelectCollection(col) => {
                if data.col != Some(col) {
                    data.col = Some(col);
                    data.folder_id = None;
                }
                Command::none()
            }
            Message::Close => {
                state.popup = None;
                Command::none()
            }
        }
    }
}

pub fn title<'a>() -> Cow<'a, str> {
    Cow::Borrowed("Save Request")
}

pub fn done(data: &SaveRequestState) -> Option<Message> {
    data.col.map(Message::Done)
}

pub fn col_selector<'a>(state: &'a AppState, data: &'a SaveRequestState) -> Element<'a, Message> {
    let collections = state
        .collections
        .entries
        .iter()
        .map(|(k, c)| {
            let name = c.name.as_str();
            button(text(name))
                .on_press(Message::SelectCollection(k))
                .style(if Some(k) == data.col {
                    button::primary
                } else {
                    button::text
                })
                .padding([2, 4])
                .into()
        })
        .collect();

    scrollable(
        Column::new()
            .push("Collection")
            .push(Column::from_vec(collections).width(Length::Shrink))
            .spacing(4)
            .padding(4)
            .align_items(iced::Alignment::Center),
    )
    .into()
}

pub fn dir_selector(collection: &Collection, folder: Option<FolderId>) -> Element<Message> {
    let children = match folder {
        Some(folder) => {
            match collection
                .children
                .iter()
                .find(|e| matches!(e, Entry::Folder(Folder { id, .. }) if *id == folder))
                .expect("folder not found")
            {
                Entry::Folder(dir) => &dir.children,
                _ => {
                    return Column::new().into();
                }
            }
        }
        _ => &collection.children,
    };

    let entries: Vec<Element<Message>> = children
        .iter()
        .filter_map(|e| match e {
            Entry::Folder(Folder { id, name, .. }) => {
                Some(button(text(name)).on_press(Message::SelectDir(*id)).into())
            }
            Entry::Item(_) => None,
        })
        .collect();

    Column::new()
        .push("Folder")
        .push(Column::from_vec(entries))
        .spacing(4)
        .padding(4)
        .align_items(iced::Alignment::Center)
        .into()
}

pub(crate) fn view<'a>(state: &'a AppState, data: &'a SaveRequestState) -> Element<'a, Message> {
    let collection = data.col.and_then(|col| state.collections.get(col));

    let name = Row::new()
        .push(text("Name"))
        .push(horizontal_space())
        .push(
            text_input("Name", &data.name)
                .on_input(Message::NameChanged)
                .on_paste(Message::NameChanged),
        )
        .spacing(4);

    let col_name = collection
        .map(|c| c.name.as_str())
        .unwrap_or("Select Collection");

    let path = Row::new()
        .push(text("Collection"))
        .push(horizontal_space())
        .push(text(col_name))
        .spacing(4);

    let folder_selector = match collection {
        Some(c) => Some(dir_selector(c, data.folder_id)),
        None => None,
    };

    let col_selector = container(
        Row::new()
            .push(col_selector(state, data))
            .push_maybe(folder_selector),
    )
    .width(Length::Fill)
    .max_height(200)
    .style(container::bordered_box);

    Column::new()
        .push(name)
        .push(path)
        .push(col_selector)
        .width(350)
        .spacing(4)
        .into()
}
