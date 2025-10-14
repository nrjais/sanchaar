use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use crate::components::editor::ContentAction;
use crate::components::{CodeEditorMsg, editor::Content};
use crate::components::{KeyFileList, KeyValList};
use crate::components::{KeyValUpdateMsg, KeyValue};
use crate::state::utils::{key_value_from_text, key_value_to_text};
use iced::advanced::widget;
use lib::http::request::{self, Auth, Method, Request, RequestBody};
use reqwest::Url;
use serde_json::Value;
use strum::{Display, EnumString, IntoStaticStr, VariantNames};

use super::utils::{from_core_kf_list, from_core_kv_list, to_core_kf_list, to_core_kv_list};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    VariantNames,
    strum::Display,
    EnumString,
    IntoStaticStr,
)]
pub enum AuthIn {
    #[default]
    Query,
    Header,
}

impl AuthIn {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl From<&AuthIn> for request::AuthIn {
    fn from(val: &AuthIn) -> Self {
        match val {
            AuthIn::Query => request::AuthIn::Query,
            AuthIn::Header => request::AuthIn::Header,
        }
    }
}

impl From<request::AuthIn> for AuthIn {
    fn from(val: request::AuthIn) -> Self {
        match val {
            request::AuthIn::Query => AuthIn::Query,
            request::AuthIn::Header => AuthIn::Header,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReqTabId {
    #[default]
    Params,
    Body,
    Auth,
    Headers,
    PreRequest,
}

#[derive(Debug, Default, Display, VariantNames, IntoStaticStr, EnumString)]
pub enum RawAuthType {
    #[default]
    #[strum(serialize = "None")]
    None,
    #[strum(serialize = "Basic Auth")]
    Basic {
        username: Content,
        password: Content,
    },
    #[strum(serialize = "Bearer Token")]
    Bearer { token: Content },
    #[strum(serialize = "API Key")]
    APIKey {
        key: Content,
        value: Content,
        add_to: AuthIn,
    },
    #[strum(serialize = "JWT Bearer")]
    JWTBearer {
        secret: Content,
        payload: Content,
        add_to: AuthIn,
    },
}

impl RawAuthType {
    fn to_auth(&self) -> Auth {
        match self {
            RawAuthType::None => Auth::None,
            RawAuthType::Basic { username, password } => Auth::Basic {
                username: username.text().trim().to_string(),
                password: password.text().trim().to_string(),
            },
            RawAuthType::Bearer { token } => Auth::Bearer {
                token: token.text().trim().to_string(),
            },
            RawAuthType::APIKey { key, value, add_to } => Auth::APIKey {
                key: key.text().trim().to_string(),
                value: value.text().trim().to_string(),
                add_to: add_to.into(),
            },
            RawAuthType::JWTBearer {
                secret,
                payload,
                add_to,
            } => Auth::JWTBearer {
                secret: secret.text().trim().to_string(),
                payload: payload.text().trim().to_string(),
                add_to: add_to.into(),
            },
        }
    }

    fn from_auth(auth: Auth) -> RawAuthType {
        match auth {
            Auth::None => RawAuthType::None,
            Auth::Basic { username, password } => RawAuthType::Basic {
                username: Content::with_text(&username),
                password: Content::with_text(&password),
            },
            Auth::Bearer { token } => RawAuthType::Bearer {
                token: Content::with_text(&token),
            },
            Auth::APIKey { key, value, add_to } => RawAuthType::APIKey {
                key: Content::with_text(&key),
                value: Content::with_text(&value),
                add_to: add_to.into(),
            },
            Auth::JWTBearer {
                secret,
                payload,
                add_to,
            } => RawAuthType::JWTBearer {
                secret: Content::with_text(&secret),
                payload: Content::with_text(&payload),
                add_to: add_to.into(),
            },
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

#[derive(Debug, Default, Display, VariantNames, IntoStaticStr, EnumString)]
pub enum RawRequestBody {
    #[default]
    #[strum(serialize = "None")]
    None,
    #[strum(serialize = "URL Encoded")]
    Form(KeyValList),
    #[strum(serialize = "Multipart")]
    Multipart(KeyValList, KeyFileList),
    #[strum(serialize = "JSON")]
    Json(Content),
    #[strum(serialize = "XML")]
    XML(Content),
    #[strum(serialize = "Text")]
    Text(Content),
    #[strum(serialize = "File")]
    File(Option<PathBuf>),
}

impl RawRequestBody {
    fn to_request_body(&self) -> RequestBody {
        match self {
            RawRequestBody::Form(form) => RequestBody::Form(to_core_kv_list(form)),
            RawRequestBody::Json(json) => RequestBody::Json(json.text()),
            RawRequestBody::XML(xml) => RequestBody::XML(xml.text()),
            RawRequestBody::Text(text) => RequestBody::Text(text.text()),
            RawRequestBody::File(file) => RequestBody::File(file.clone()),
            RawRequestBody::Multipart(params, files) => RequestBody::Multipart {
                params: to_core_kv_list(params),
                files: to_core_kf_list(files),
            },
            RawRequestBody::None => RequestBody::None,
        }
    }

    fn from_request_body(body: RequestBody) -> RawRequestBody {
        match body {
            RequestBody::Form(form) => RawRequestBody::Form(from_core_kv_list(&form, false)),
            RequestBody::Json(json) => RawRequestBody::Json(Content::with_text(&json)),
            RequestBody::XML(xml) => RawRequestBody::XML(Content::with_text(&xml)),
            RequestBody::Text(text) => RawRequestBody::Text(Content::with_text(&text)),
            RequestBody::File(file) => RawRequestBody::File(file.clone()),
            RequestBody::Multipart { params, files } => RawRequestBody::Multipart(
                from_core_kv_list(&params, false),
                from_core_kf_list(files),
            ),
            RequestBody::None => RawRequestBody::None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

#[derive(Debug)]
pub enum BulkEditable {
    KeyValue(KeyValList),
    Editor(Content),
}

#[derive(Debug, Clone)]
pub enum BulkEditMsg {
    KeyValue(KeyValUpdateMsg),
    Editor(CodeEditorMsg),
    ToggleMode,
}

impl BulkEditable {
    pub fn key_value(keys: KeyValList) -> Self {
        Self::KeyValue(keys)
    }

    pub fn editor(content: Content) -> Self {
        Self::Editor(content)
    }

    pub fn is_editor(&self) -> bool {
        matches!(self, Self::Editor(_))
    }

    pub fn toggle(&mut self) {
        match self {
            Self::KeyValue(keys) => {
                let text = key_value_to_text(keys);
                *self = Self::Editor(Content::with_text(&text));
            }
            Self::Editor(content) => {
                let text = content.text();
                let list = key_value_from_text(&text);
                *self = Self::KeyValue(list);
            }
        }
    }

    fn to_core_kv_list(&self) -> lib::http::KeyValList {
        match self {
            Self::KeyValue(keys) => to_core_kv_list(keys),
            Self::Editor(content) => {
                let text = content.text();
                let list = key_value_from_text(&text);
                to_core_kv_list(&list)
            }
        }
    }

    pub fn update(&mut self, msg: BulkEditMsg) {
        match (msg, self) {
            (BulkEditMsg::KeyValue(msg), BulkEditable::KeyValue(keys)) => {
                keys.update(msg);
            }
            (BulkEditMsg::Editor(msg), BulkEditable::Editor(content)) => {
                msg.update(content);
            }
            (BulkEditMsg::ToggleMode, this) => {
                this.toggle();
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct RequestPane {
    pub url_id: widget::Id,
    pub url_content: Content,
    pub method: Method,
    pub headers: BulkEditable,
    pub body: RawRequestBody,
    pub query_params: BulkEditable,
    pub path_params: KeyValList,
    pub auth: RawAuthType,
    pub tab: ReqTabId,
    pub body_cache: HashMap<&'static str, RawRequestBody>,
    pub pre_request: Option<String>,
}

impl RequestPane {
    pub(crate) fn change_body_type(&mut self, content_type: &str) {
        let new_body = self.body_cache.remove(content_type).unwrap_or_else(|| {
            RawRequestBody::from_str(content_type).unwrap_or(RawRequestBody::None)
        });
        let old_body = std::mem::replace(&mut self.body, new_body);
        self.body_cache.insert(old_body.as_str(), old_body);
    }

    pub fn change_auth_type(&mut self, auth_type: &str) {
        self.auth = RawAuthType::from_str(auth_type).unwrap_or(RawAuthType::None);
    }

    pub fn to_request(&self) -> Request {
        Request {
            description: "Http request".to_string(),
            method: self.method,
            url: self.url_content.text().trim().to_string(),
            headers: self.headers.to_core_kv_list(),
            body: self.body.to_request_body(),
            auth: self.auth.to_auth(),
            query_params: self.query_params.to_core_kv_list(),
            path_params: to_core_kv_list(&self.path_params),
            assertions: Default::default(),
            pre_request: self.pre_request.clone(),
        }
    }

    pub fn from(request: Request) -> RequestPane {
        RequestPane {
            url_id: widget::Id::unique(),
            url_content: Content::with_text(&request.url),
            method: request.method,
            headers: BulkEditable::key_value(from_core_kv_list(&request.headers, false)),
            body: RawRequestBody::from_request_body(request.body),
            auth: RawAuthType::from_auth(request.auth),
            query_params: BulkEditable::key_value(from_core_kv_list(&request.query_params, false)),
            path_params: from_core_kv_list(&request.path_params, true),
            tab: ReqTabId::Params,
            body_cache: HashMap::new(),
            pre_request: request.pre_request,
        }
    }

    pub fn format_body(&mut self) {
        if let RawRequestBody::Json(content) = &mut self.body {
            let text = content.text();
            let json =
                serde_json::from_str::<Value>(&text).and_then(|j| serde_json::to_string_pretty(&j));
            if let Ok(formatted) = json {
                *content = Content::with_text(&formatted);
            }
        }
    }

    pub fn clean_url(&mut self) {
        let url = self.url_content.text();
        let url = url.replace("\n", "").replace("\r", "").trim().to_string();
        if url != self.url_content.text() {
            self.url_content.perform(ContentAction::Replace(url));
        }
        // self.extract_query_params();
    }

    pub fn update_from_curl(&mut self, request: request::Request) {
        self.method = request.method;

        self.url_content
            .perform(ContentAction::Replace(request.url));

        let parsed_headers = from_core_kv_list(&request.headers, false);
        self.headers = BulkEditable::key_value(parsed_headers);

        let new_body = RawRequestBody::from_request_body(request.body);
        self.body = new_body;

        self.query_params = BulkEditable::key_value(KeyValList::new());
        self.auth = RawAuthType::from_auth(request.auth);
    }

    pub fn extract_query_params(&mut self) {
        let url_text = self.url_content.text();
        let Ok(mut url) = Url::parse(&url_text) else {
            return;
        };

        let mut query_params = Vec::new();
        for (key, value) in url.query_pairs() {
            query_params.push(KeyValue::new(&key, &value, false));
        }
        url.set_query(None);

        let url = url.to_string();
        if url != url_text {
            self.url_content.perform(ContentAction::Replace(url));
            self.query_params = BulkEditable::key_value(KeyValList::from(query_params, false));
        }
    }
}
