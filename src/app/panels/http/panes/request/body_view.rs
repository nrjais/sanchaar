use super::{RequestPaneMsg, body_editor};
use crate::components::{
    ContentType, KeyFileList, KeyValList, icon, icon_button, icons, key_value_editor,
    multi_file_picker, tooltip,
};
use crate::state::request::RawRequestBody;
use iced::{
    Element, Length,
    widget::{Column, Row, button, center, container, pick_list, scrollable, text},
};
use std::{collections::HashSet, path::PathBuf, sync::Arc};

pub fn body_tab(
    body: &RawRequestBody,
    vars: Arc<HashSet<String>>,
) -> iced::Element<RequestPaneMsg> {
    let actions = match body {
        RawRequestBody::Json(_) | RawRequestBody::XML(_) => Some(tooltip(
            "Prettify",
            icon_button(icons::Wand, None, Some(4))
                .style(button::text)
                .on_press(RequestPaneMsg::FormatBody),
        )),
        _ => None,
    };

    let header = Row::new()
        .push(text("Content Type"))
        .push(
            pick_list(
                RawRequestBody::all_variants(),
                Some(body.as_str()),
                RequestPaneMsg::ChangeBodyType,
            )
            .padding([2, 6]),
        )
        .push(actions)
        .spacing(16)
        .height(Length::Shrink)
        .align_y(iced::Alignment::Center);

    let body = match body {
        RawRequestBody::Json(content) => body_editor::view(content, ContentType::Json),
        RawRequestBody::XML(content) => body_editor::view(content, ContentType::XML),
        RawRequestBody::Text(content) => body_editor::view(content, ContentType::Text),
        RawRequestBody::Form(values) => form(values, Arc::clone(&vars)),
        RawRequestBody::Multipart(values, files) => multipart_editor(values, files, vars),
        RawRequestBody::File(path) => file(path),
        RawRequestBody::None => no_body(),
    };

    Column::new()
        .push(header)
        .push(center(body))
        .spacing(8)
        .padding([4, 0])
        .into()
}

fn file(path: &Option<PathBuf>) -> Element<RequestPaneMsg> {
    let location = path
        .as_ref()
        .map(|p| p.to_str().unwrap_or("Invalid File Path"))
        .unwrap_or("No File Selected");

    Column::new()
        .push(text(location))
        .push(
            button(text("Select File"))
                .padding([4, 12])
                .on_press(RequestPaneMsg::OpenFilePicker)
                .style(button::secondary),
        )
        .align_x(iced::Alignment::Center)
        .spacing(8)
        .into()
}

fn form(values: &KeyValList, vars: Arc<HashSet<String>>) -> Element<RequestPaneMsg> {
    scrollable(key_value_editor(values, &vars).on_change(RequestPaneMsg::FormBodyEditAction))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn no_body<'a>() -> Element<'a, RequestPaneMsg> {
    Column::new()
        .push(container(icon(icons::FileCancel).size(80.0)).padding(10))
        .push(text("No Body Content"))
        .align_x(iced::Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
}

fn multipart_editor<'a>(
    values: &'a KeyValList,
    files: &'a KeyFileList,
    vars: Arc<HashSet<String>>,
) -> Element<'a, RequestPaneMsg> {
    let params = Column::new()
        .push("Params")
        .push(key_value_editor(values, &vars).on_change(RequestPaneMsg::MultipartParamsAction))
        .width(Length::Fill)
        .spacing(4);

    let file_picker = Column::new()
        .push("Files")
        .push(multi_file_picker(files).map(RequestPaneMsg::MultipartFilesAction))
        .width(Length::Fill)
        .spacing(4);

    scrollable(Column::new().push(params).push(file_picker).spacing(8))
        .height(Length::Fill)
        .into()
}
