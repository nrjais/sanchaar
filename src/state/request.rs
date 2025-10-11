use std::collections::HashMap;
use std::path::PathBuf;

use crate::components::CodeEditorMsg;
use crate::components::KeyFileList;
use crate::components::KeyValList;
use crate::components::KeyValUpdateMsg;
use crate::components::KeyValue;
use crate::components::editor::Content;
use crate::components::editor::ContentAction;
use crate::state::utils::key_value_from_text;
use crate::state::utils::key_value_to_text;
use body_types::*;
use core::http::request::{Auth, Method, Request, RequestBody};
use iced::advanced::widget;
use reqwest::Url;
use serde_json::Value;

use super::utils::{from_core_kf_list, from_core_kv_list, to_core_kf_list, to_core_kv_list};

pub mod body_types {
    pub const FORM: &str = "URL Encoded";
    pub const MULTIPART: &str = "Multipart";
    pub const JSON: &str = "JSON";
    pub const XML: &str = "XML";
    pub const TEXT: &str = "Text";
    pub const FILE: &str = "File";
    pub const NONE: &str = "None";
}

pub mod auth_types {
    pub const NONE: &str = "None";
    pub const BASIC: &str = "Basic Auth";
    pub const BEARER: &str = "Bearer Token";
    pub const API_KEY: &str = "API Key";
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

#[derive(Debug, Default)]
pub enum RawAuthType {
    #[default]
    None,
    Basic {
        username: Content,
        password: Content,
    },
    Bearer {
        token: Content,
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
        }
    }

    pub fn as_str(&self) -> &'static str {
        use auth_types::*;
        match self {
            RawAuthType::Basic { .. } => BASIC,
            RawAuthType::Bearer { .. } => BEARER,
            RawAuthType::None => NONE,
        }
    }

    pub fn all_variants() -> &'static [&'static str] {
        use auth_types::*;
        &[NONE, BASIC, BEARER]
    }
}

#[derive(Debug, Default)]
pub enum RawRequestBody {
    #[default]
    None,
    Form(KeyValList),
    Multipart(KeyValList, KeyFileList),
    Json(Content),
    XML(Content),
    Text(Content),
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
}

impl RawRequestBody {
    pub fn as_str(&self) -> &'static str {
        match self {
            RawRequestBody::Form(_) => FORM,
            RawRequestBody::Json(_) => JSON,
            RawRequestBody::XML(_) => XML,
            RawRequestBody::Text(_) => TEXT,
            RawRequestBody::File(_) => FILE,
            RawRequestBody::None => NONE,
            RawRequestBody::Multipart(_, _) => MULTIPART,
        }
    }

    pub fn all_variants() -> &'static [&'static str] {
        &[FORM, MULTIPART, JSON, XML, TEXT, FILE, NONE]
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

    fn to_core_kv_list(&self) -> core::http::KeyValList {
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
        let new_body = self
            .body_cache
            .remove(content_type)
            .unwrap_or_else(|| match content_type {
                FORM => RawRequestBody::Form(KeyValList::new()),
                JSON => RawRequestBody::Json(Default::default()),
                XML => RawRequestBody::XML(Default::default()),
                TEXT => RawRequestBody::Text(Default::default()),
                FILE => RawRequestBody::File(Default::default()),
                MULTIPART => RawRequestBody::Multipart(KeyValList::new(), KeyFileList::new()),
                _ => RawRequestBody::None,
            });
        let old_body = std::mem::replace(&mut self.body, new_body);
        self.body_cache.insert(old_body.as_str(), old_body);
    }

    pub fn change_auth_type(&mut self, auth_type: &str) {
        self.auth = match auth_type {
            auth_types::NONE => RawAuthType::None,
            auth_types::BASIC => RawAuthType::Basic {
                username: Content::new(),
                password: Content::new(),
            },
            auth_types::BEARER => RawAuthType::Bearer {
                token: Content::new(),
            },
            _ => RawAuthType::None,
        };
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

    pub fn update_from_curl(&mut self, request: core::http::request::Request) {
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
