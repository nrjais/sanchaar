use std::{collections::HashSet, str::FromStr, sync::Arc};

use crate::{
    components::{
        CodeEditorMsg, ContentType, LineEditorMsg, code_editor, editor, icon, icons, line_editor,
    },
    state::request::AuthIn,
};
use iced::{
    Element,
    Length::{self},
    widget::{Column, Row, center, container, pick_list, text},
};
use strum::VariantNames;

use crate::state::request::{RawAuthType, RequestPane};

#[derive(Debug, Clone)]
pub enum AuthEditorMsg {
    ChangeAuthType(&'static str),
    BearerToken(LineEditorMsg),
    BasicUsername(LineEditorMsg),
    BasicPassword(LineEditorMsg),
    APIKeyName(LineEditorMsg),
    APIKeyValue(LineEditorMsg),
    APIKeyAddTo(&'static str),
    JWTBearerAlgorithm(&'static str),
    JWTBearerSecret(LineEditorMsg),
    JWTBearerPayload(CodeEditorMsg),
    JWTBearerAddTo(&'static str),
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
            AuthEditorMsg::APIKeyName(line_editor_msg) => {
                if let RawAuthType::APIKey { key, .. } = &mut request.auth {
                    line_editor_msg.update(key);
                }
            }
            AuthEditorMsg::APIKeyValue(line_editor_msg) => {
                if let RawAuthType::APIKey { value, .. } = &mut request.auth {
                    line_editor_msg.update(value);
                }
            }
            AuthEditorMsg::APIKeyAddTo(update) => {
                if let RawAuthType::APIKey { add_to, .. } = &mut request.auth {
                    *add_to = AuthIn::from_str(update).unwrap_or(AuthIn::Header);
                }
            }
            AuthEditorMsg::JWTBearerAlgorithm(algo) => {
                if let RawAuthType::JWTBearer { algorithm, .. } = &mut request.auth {
                    use crate::state::request::JwtAlgo;
                    *algorithm = JwtAlgo::from_str(algo).unwrap_or(JwtAlgo::HS256);
                }
            }
            AuthEditorMsg::JWTBearerSecret(line_editor_msg) => {
                if let RawAuthType::JWTBearer { secret, .. } = &mut request.auth {
                    line_editor_msg.update(secret);
                }
            }
            AuthEditorMsg::JWTBearerPayload(line_editor_msg) => {
                if let RawAuthType::JWTBearer { payload, .. } = &mut request.auth {
                    line_editor_msg.update(payload);
                }
            }
            AuthEditorMsg::JWTBearerAddTo(update) => {
                if let RawAuthType::JWTBearer { add_to, .. } = &mut request.auth {
                    *add_to = AuthIn::from_str(update).unwrap_or(AuthIn::Header);
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
        .push(text(label).width(Length::FillPortion(2)))
        .push(container(field).width(Length::FillPortion(3)))
        .into()
}

pub fn auth_view(request: &RequestPane, vars: Arc<HashSet<String>>) -> Element<AuthEditorMsg> {
    let auth = &request.auth;
    let header = Row::new()
        .push(text("Auth Type"))
        .push(
            pick_list(
                RawAuthType::VARIANTS,
                Some(auth.as_str()),
                AuthEditorMsg::ChangeAuthType,
            )
            .padding([2, 6]),
        )
        .spacing(16)
        .height(Length::Shrink)
        .align_y(iced::Alignment::Center);

    let body: Element<AuthEditorMsg> = auth_body(auth, vars);

    Column::new()
        .push(header)
        .push(center(body))
        .spacing(12)
        .into()
}

fn auth_body(auth: &RawAuthType, vars: Arc<HashSet<String>>) -> Element<AuthEditorMsg> {
    match auth {
        RawAuthType::Basic { username, password } => Column::new()
            .push(field_row(
                "Username",
                line_editor(username)
                    .vars(Arc::clone(&vars))
                    .map(AuthEditorMsg::BasicUsername),
            ))
            .push(field_row(
                "Password",
                line_editor(password)
                    .vars(Arc::clone(&vars))
                    .map(AuthEditorMsg::BasicPassword),
            ))
            .height(Length::Fill)
            .spacing(4)
            .into(),
        RawAuthType::Bearer { token } => Column::new()
            .push(field_row(
                "Token",
                line_editor(token)
                    .vars(Arc::clone(&vars))
                    .map(AuthEditorMsg::BearerToken),
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
        RawAuthType::APIKey { key, value, add_to } => api_key_view(key, value, *add_to),
        RawAuthType::JWTBearer {
            algorithm,
            secret,
            payload,
            add_to,
        } => jwt_bearer_view(*algorithm, secret, payload, *add_to),
    }
}

fn api_key_view<'a>(
    key: &'a editor::Content,
    value: &'a editor::Content,
    add_to: AuthIn,
) -> Element<'a, AuthEditorMsg> {
    Column::new()
        .push(field_row(
            "Add To",
            pick_list(
                AuthIn::VARIANTS,
                Some(add_to.as_str()),
                AuthEditorMsg::APIKeyAddTo,
            ),
        ))
        .push(field_row(
            "Name",
            line_editor(key).map(AuthEditorMsg::APIKeyName),
        ))
        .push(field_row(
            "Key",
            line_editor(value).map(AuthEditorMsg::APIKeyValue),
        ))
        .height(Length::Fill)
        .spacing(4)
        .into()
}

fn jwt_bearer_view<'a>(
    algorithm: crate::state::request::JwtAlgo,
    secret: &'a editor::Content,
    payload: &'a editor::Content,
    add_to: AuthIn,
) -> Element<'a, AuthEditorMsg> {
    use crate::state::request::JwtAlgo;

    Column::new()
        .push(field_row(
            "Algorithm",
            pick_list(
                JwtAlgo::VARIANTS,
                Some(algorithm.as_str()),
                AuthEditorMsg::JWTBearerAlgorithm,
            ),
        ))
        .push(field_row(
            "Add To",
            pick_list(
                AuthIn::VARIANTS,
                Some(add_to.as_str()),
                AuthEditorMsg::JWTBearerAddTo,
            ),
        ))
        .push(field_row(
            "Secret",
            line_editor(secret).map(AuthEditorMsg::JWTBearerSecret),
        ))
        .push(field_row(
            "Payload",
            code_editor(payload, ContentType::Json)
                .editable()
                .map(AuthEditorMsg::JWTBearerPayload),
        ))
        .height(Length::Fill)
        .spacing(4)
        .into()
}
