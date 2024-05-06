use super::{body_editor, RequestPaneMsg};
use crate::state::request::RawRequestBody;
use components::{
    icon, icons, key_value_editor, multi_file_picker, ContentType, KeyFileList, KeyValList,
};
use iced::{
    widget::{button, container, horizontal_space, pick_list, scrollable, text, Column, Row},
    Element, Length,
};
use std::path::PathBuf;

pub fn body_tab(body: &RawRequestBody) -> iced::Element<RequestPaneMsg> {
    let size = 14;
    let header = Row::new()
        .push(text(format!("Content Type: {}", body.as_str())).size(size))
        .push(horizontal_space())
        .push(
            pick_list(
                RawRequestBody::all_variants(),
                Some(body.as_str()),
                RequestPaneMsg::ChangeBodyType,
            )
            .text_size(size)
            .padding([2, 4]),
        )
        .height(Length::Shrink)
        .align_items(iced::Alignment::Center);

    let body = match body {
        RawRequestBody::Json(content) => body_editor::view(content, ContentType::Json),
        RawRequestBody::XML(content) => body_editor::view(content, ContentType::XML),
        RawRequestBody::Text(content) => body_editor::view(content, ContentType::Text),
        RawRequestBody::Form(values) => form(values),
        RawRequestBody::Multipart(values, files) => multipart_editor(values, files),
        RawRequestBody::File(path) => file(path),
        RawRequestBody::None => no_body(),
    };

    Column::new()
        .push(header)
        .push(
            container(body)
                .height(Length::Fill)
                .width(Length::Fill)
                .center_x()
                .center_y(),
        )
        .spacing(8)
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
        .align_items(iced::Alignment::Center)
        .spacing(8)
        .into()
}

fn form(values: &KeyValList) -> Element<RequestPaneMsg> {
    scrollable(key_value_editor(values).on_change(RequestPaneMsg::FormBodyEditAction))
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn no_body<'a>() -> Element<'a, RequestPaneMsg> {
    Column::new()
        .push(container(icon(icons::FileCancel).size(80.0)).padding(10))
        .push(text("No Body Content"))
        .align_items(iced::Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
}

fn multipart_editor<'a>(
    values: &'a KeyValList,
    files: &'a KeyFileList,
) -> Element<'a, RequestPaneMsg> {
    let params = Column::new()
        .push("Params")
        .push(key_value_editor(values).on_change(RequestPaneMsg::MultipartParamsAction))
        .width(Length::Fill)
        .spacing(4);

    let file_picker = Column::new()
        .push("Files")
        .push(
            multi_file_picker(files, RequestPaneMsg::MulitpartOpenFilePicker)
                .on_change(RequestPaneMsg::MultipartFilesAction),
        )
        .width(Length::Fill)
        .spacing(4);

    scrollable(Column::new().push(params).push(file_picker).spacing(8))
        .height(Length::Fill)
        .into()
}