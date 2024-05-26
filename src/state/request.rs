use std::collections::HashMap;
use std::path::PathBuf;

use body_types::*;
use components::{self, KeyFileList};
use components::{text_editor, KeyValList};
use core::http::request::{Auth, Method, Request, RequestBody};

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
    pub const BASIC: &str = "Basic";
    pub const BEARER: &str = "Bearer";
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
        username: text_editor::Content,
        password: text_editor::Content,
    },
    Bearer {
        token: text_editor::Content,
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
                username: text_editor::Content::with_text(&username),
                password: text_editor::Content::with_text(&password),
            },
            Auth::Bearer { token } => RawAuthType::Bearer {
                token: text_editor::Content::with_text(&token),
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
    Json(text_editor::Content),
    XML(text_editor::Content),
    Text(text_editor::Content),
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
            RequestBody::Form(form) => RawRequestBody::Form(from_core_kv_list(form, false)),
            RequestBody::Json(json) => RawRequestBody::Json(text_editor::Content::with_text(&json)),
            RequestBody::XML(xml) => RawRequestBody::XML(text_editor::Content::with_text(&xml)),
            RequestBody::Text(text) => RawRequestBody::Text(text_editor::Content::with_text(&text)),
            RequestBody::File(file) => RawRequestBody::File(file.clone()),
            RequestBody::Multipart { params, files } => RawRequestBody::Multipart(
                from_core_kv_list(params, false),
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
pub struct RequestPane {
    pub url_content: text_editor::Content,
    pub method: Method,
    pub headers: KeyValList,
    pub body: RawRequestBody,
    pub query_params: KeyValList,
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
                username: text_editor::Content::new(),
                password: text_editor::Content::new(),
            },
            auth_types::BEARER => RawAuthType::Bearer {
                token: text_editor::Content::new(),
            },
            _ => RawAuthType::None,
        };
    }

    pub fn to_request(&self) -> Request {
        Request {
            description: "Http request".to_string(),
            method: self.method,
            url: self.url_content.text().trim().to_string(),
            headers: to_core_kv_list(&self.headers),
            body: self.body.to_request_body(),
            auth: self.auth.to_auth(),
            query_params: to_core_kv_list(&self.query_params),
            path_params: to_core_kv_list(&self.path_params),
            pre_request: self.pre_request.clone(),
        }
    }

    pub fn from(request: Request) -> RequestPane {
        RequestPane {
            url_content: text_editor::Content::with_text(&request.url),
            method: request.method,
            headers: from_core_kv_list(request.headers, false),
            body: RawRequestBody::from_request_body(request.body),
            auth: RawAuthType::from_auth(request.auth),
            query_params: from_core_kv_list(request.query_params, false),
            path_params: from_core_kv_list(request.path_params, true),
            tab: ReqTabId::Params,
            body_cache: HashMap::new(),
            pre_request: request.pre_request,
        }
    }
}
