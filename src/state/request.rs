use std::collections::HashMap;
use std::path::PathBuf;

use body_types::*;
use components;
use components::{text_editor, KeyValList};
use core::http::request::{Method, Request, RequestBody};

use super::utils::{from_core_kv_list, to_core_kv_list};

pub mod body_types {
    pub const FORM: &str = "URL Encoded";
    pub const JSON: &str = "Json";
    pub const XML: &str = "XML";
    pub const TEXT: &str = "Text";
    pub const FILE: &str = "File";
    pub const NONE: &str = "None";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReqTabId {
    #[default]
    Params,
    Body,
    Headers,
}

#[derive(Debug, Default)]
pub enum RawRequestBody {
    Form(KeyValList),
    Json(text_editor::Content),
    XML(text_editor::Content),
    Text(text_editor::Content),
    File(Option<PathBuf>),
    #[default]
    None,
}

impl RawRequestBody {
    fn to_request_body(&self) -> RequestBody {
        match self {
            RawRequestBody::Form(form) => RequestBody::Form(to_core_kv_list(form)),
            RawRequestBody::Json(json) => RequestBody::Json(json.text()),
            RawRequestBody::XML(xml) => RequestBody::XML(xml.text()),
            RawRequestBody::Text(text) => RequestBody::Text(text.text()),
            RawRequestBody::File(file) => RequestBody::File(file.clone()),
            RawRequestBody::None => RequestBody::None,
        }
    }

    fn from_request_body(body: &RequestBody) -> RawRequestBody {
        match body {
            RequestBody::Form(form) => RawRequestBody::Form(from_core_kv_list(form.clone(), false)),
            RequestBody::Json(json) => RawRequestBody::Json(text_editor::Content::with_text(json)),
            RequestBody::XML(xml) => RawRequestBody::XML(text_editor::Content::with_text(xml)),
            RequestBody::Text(text) => RawRequestBody::Text(text_editor::Content::with_text(text)),
            RequestBody::File(file) => RawRequestBody::File(file.clone()),
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
        }
    }

    pub fn all_variants() -> &'static [&'static str] {
        &[FORM, JSON, XML, TEXT, FILE, NONE]
    }
}

#[derive(Debug)]
pub struct RequestPane {
    pub description: String,
    pub url: String,
    pub url_content: text_editor::Content,
    pub method: Method,
    pub headers: KeyValList,
    pub body: RawRequestBody,
    pub query_params: KeyValList,
    pub path_params: KeyValList,
    pub tab: ReqTabId,
    pub body_cache: HashMap<&'static str, RawRequestBody>,
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
                NONE => RawRequestBody::None,
                _ => RawRequestBody::None,
            });
        let old_body = std::mem::replace(&mut self.body, new_body);
        self.body_cache.insert(old_body.as_str(), old_body);
    }

    pub fn to_request(&self) -> Request {
        Request {
            description: self.description.clone(),
            method: self.method,
            url: self.url.clone(),
            headers: to_core_kv_list(&self.headers),
            body: self.body.to_request_body(),
            query_params: to_core_kv_list(&self.query_params),
            path_params: to_core_kv_list(&self.path_params),
        }
    }

    pub fn from(request: Request) -> RequestPane {
        RequestPane {
            description: request.description,
            url_content: text_editor::Content::with_text(&request.url),
            url: request.url,
            method: request.method,
            headers: from_core_kv_list(request.headers, false),
            body: RawRequestBody::from_request_body(&request.body),
            query_params: from_core_kv_list(request.query_params, false),
            path_params: from_core_kv_list(request.path_params, true),
            tab: ReqTabId::Params,
            body_cache: HashMap::new(),
        }
    }
}
