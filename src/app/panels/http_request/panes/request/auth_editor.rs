use components::{
    icon, icons,
    text_editor::{line_editor, ContentAction},
};
use iced::{
    widget::{center, container, horizontal_space, pick_list, text, Column, Row},
    Element, Length,
};

use crate::state::request::{RawAuthType, RequestPane};

#[derive(Debug, Clone)]
pub enum AuthEditorMsg {
    ChangeAuthType(&'static str),
    BearerToken(ContentAction),
    BasicUsername(ContentAction),
    BasicPassword(ContentAction),
}
impl AuthEditorMsg {
    pub(crate) fn update(self, request: &mut RequestPane) {
        match self {
            AuthEditorMsg::ChangeAuthType(auth) => request.change_auth_type(auth),
            AuthEditorMsg::BearerToken(action) => {
                if let RawAuthType::Bearer { token } = &mut request.auth {
                    token.perform(action);
                }
            }
            AuthEditorMsg::BasicUsername(action) => {
                if let RawAuthType::Basic { username, .. } = &mut request.auth {
                    username.perform(action);
                }
            }
            AuthEditorMsg::BasicPassword(action) => {
                if let RawAuthType::Basic { password, .. } = &mut request.auth {
                    password.perform(action);
                }
            }
        }
    }
}

fn field_row<'a>(
    label: &'static str,
    field: impl Into<Element<'a, AuthEditorMsg>>,
) -> Element<'a, AuthEditorMsg> {
    Row::new()
        .push(text(label))
        .push(horizontal_space())
        .push(field)
        .into()
}

pub fn auth_view(request: &RequestPane) -> Element<AuthEditorMsg> {
    let auth = &request.auth;
    let size = 14;
    let header = Row::new()
        .push(text(format!("Auth Method: {}", auth.as_str())).size(size))
        .push(horizontal_space())
        .push(
            pick_list(
                RawAuthType::all_variants(),
                Some(auth.as_str()),
                AuthEditorMsg::ChangeAuthType,
            )
            .text_size(size)
            .padding([2, 4]),
        )
        .height(Length::Shrink)
        .align_items(iced::Alignment::Center);

    let body: Element<AuthEditorMsg> = auth_body(auth);

    Column::new()
        .push(header)
        .push(center(body).padding(8))
        .spacing(4)
        .into()
}

fn auth_body(auth: &RawAuthType) -> Element<AuthEditorMsg> {
    match auth {
        RawAuthType::Basic { username, password } => Column::new()
            .push(field_row(
                "Username",
                line_editor(username).on_action(AuthEditorMsg::BasicUsername),
            ))
            .push(field_row(
                "Password",
                line_editor(password).on_action(AuthEditorMsg::BasicPassword),
            ))
            .height(Length::Fill)
            .spacing(4)
            .into(),
        RawAuthType::Bearer { token } => Column::new()
            .push(field_row(
                "Token",
                line_editor(token).on_action(AuthEditorMsg::BearerToken),
            ))
            .height(Length::Fill)
            .spacing(4)
            .into(),
        RawAuthType::None => {
            let empty_icon = container(icon(icons::FileCancel).size(80.0)).padding(10);
            Column::new()
                .push(empty_icon)
                .push(text("No Auth"))
                .align_items(iced::Alignment::Center)
                .height(Length::Shrink)
                .width(Length::Shrink)
                .into()
        }
    }
}
