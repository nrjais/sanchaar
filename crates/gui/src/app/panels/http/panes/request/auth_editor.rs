use std::{collections::HashSet, sync::Arc};

use components::{icon, icons, line_editor, LineEditorMsg};
use iced::{
    widget::{center, container, horizontal_space, pick_list, text, Column, Row},
    Element, Length,
};

use crate::state::request::{RawAuthType, RequestPane};

#[derive(Debug, Clone)]
pub enum AuthEditorMsg {
    ChangeAuthType(&'static str),
    BearerToken(LineEditorMsg),
    BasicUsername(LineEditorMsg),
    BasicPassword(LineEditorMsg),
}
impl AuthEditorMsg {
    pub(crate) fn update(self, request: &mut RequestPane) {
        match self {
            AuthEditorMsg::ChangeAuthType(auth) => request.change_auth_type(auth),
            AuthEditorMsg::BearerToken(action) => {
                if let RawAuthType::Bearer { token } = &mut request.auth {
                    action.update(token);
                }
            }
            AuthEditorMsg::BasicUsername(action) => {
                if let RawAuthType::Basic { username, .. } = &mut request.auth {
                    action.update(username);
                }
            }
            AuthEditorMsg::BasicPassword(action) => {
                if let RawAuthType::Basic { password, .. } = &mut request.auth {
                    action.update(password);
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

pub fn auth_view(request: &RequestPane, vars: Arc<HashSet<String>>) -> Element<AuthEditorMsg> {
    let auth = &request.auth;
    let header = Row::new()
        .push(text("Auth Method"))
        .push(horizontal_space())
        .push(
            pick_list(
                RawAuthType::all_variants(),
                Some(auth.as_str()),
                AuthEditorMsg::ChangeAuthType,
            )
            .padding([2, 8]),
        )
        .height(Length::Shrink)
        .align_y(iced::Alignment::Center);

    let body: Element<AuthEditorMsg> = auth_body(auth, vars);

    Column::new()
        .push(header)
        .push(center(body).padding(8))
        .spacing(4)
        .into()
}

fn auth_body(auth: &RawAuthType, vars: Arc<HashSet<String>>) -> Element<AuthEditorMsg> {
    match auth {
        RawAuthType::Basic { username, password } => Column::new()
            .push(field_row(
                "Username",
                line_editor(username)
                    .on_action(AuthEditorMsg::BasicUsername)
                    .vars(Arc::clone(&vars)),
            ))
            .push(field_row(
                "Password",
                line_editor(password)
                    .on_action(AuthEditorMsg::BasicPassword)
                    .vars(Arc::clone(&vars)),
            ))
            .height(Length::Fill)
            .spacing(4)
            .into(),
        RawAuthType::Bearer { token } => Column::new()
            .push(field_row(
                "Token",
                line_editor(token)
                    .on_action(AuthEditorMsg::BearerToken)
                    .vars(Arc::clone(&vars)),
            ))
            .height(Length::Fill)
            .spacing(4)
            .into(),
        RawAuthType::None => {
            let empty_icon = container(icon(icons::FileCancel).size(80.0)).padding(10);
            Column::new()
                .push(empty_icon)
                .push(text("No Auth"))
                .align_x(iced::Alignment::Center)
                .height(Length::Shrink)
                .width(Length::Shrink)
                .into()
        }
    }
}
