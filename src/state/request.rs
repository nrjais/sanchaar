use std::path::PathBuf;

use iced::widget::text_editor;
use strum::{Display, EnumString, VariantArray};

use crate::components::KeyValList;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReqTabId {
    #[default]
    Params,
    Body,
    Headers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, VariantArray, Display, Default)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
}

#[derive(Debug, Default)]
pub enum RequestRawBody {
    Form(KeyValList),
    Json(text_editor::Content),
    XML(text_editor::Content),
    Text(text_editor::Content),
    File(PathBuf),
    #[default]
    None,
}

impl RequestRawBody {
    fn to_request_body(&self) -> RequestBody {
        match self {
            RequestRawBody::Form(form) => RequestBody::Form(form.clone()),
            RequestRawBody::Json(json) => RequestBody::Json(json.text()),
            RequestRawBody::XML(xml) => RequestBody::XML(xml.text()),
            RequestRawBody::Text(text) => RequestBody::Text(text.text()),
            RequestRawBody::File(file) => RequestBody::File(file.clone()),
            RequestRawBody::None => RequestBody::None,
        }
    }
}

impl RequestRawBody {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestRawBody::Form(_) => "URL Encoded",
            RequestRawBody::Json(_) => "Json",
            RequestRawBody::XML(_) => "XML",
            RequestRawBody::Text(_) => "Text",
            RequestRawBody::File(_) => "File",
            RequestRawBody::None => "None",
        }
    }

    pub fn all_variants() -> &'static [&'static str] {
        &["URL Encoded", "Json", "XML", "Text", "File", "None"]
    }
}

#[derive(Debug)]
pub struct RequestPane {
    pub description: String,
    pub url: String,
    pub method: Method,
    pub headers: KeyValList,
    pub body: RequestRawBody,
    pub query_params: KeyValList,
    pub path_params: KeyValList,
    pub tab: ReqTabId,
}

#[derive(Debug, Clone)]
pub enum RequestBody {
    Form(KeyValList),
    Json(String),
    XML(String),
    Text(String),
    File(PathBuf),
    None,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub description: String,
    pub method: Method,
    pub url: String,
    pub headers: KeyValList,
    pub body: RequestBody,
    pub query_params: KeyValList,
    pub path_params: KeyValList,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            description: "Http request".to_string(),
            method: Method::GET,
            url: "https://echo.nrjais.com".to_string(),
            headers: KeyValList::new(),
            body: RequestBody::None,
            query_params: KeyValList::new(),
            path_params: KeyValList::empty(),
        }
    }
}

impl RequestPane {
    pub fn to_request(&self) -> Request {
        Request {
            description: self.description.clone(),
            method: self.method,
            url: self.url.clone(),
            headers: self.headers.clone(),
            body: self.body.to_request_body(),
            query_params: self.query_params.clone(),
            path_params: self.path_params.clone(),
        }
    }

    pub fn from(request: Request) -> RequestPane {
        RequestPane {
            description: request.description,
            url: request.url,
            method: request.method,
            headers: request.headers,
            body: RequestRawBody::None,
            query_params: request.query_params,
            path_params: request.path_params,
            tab: ReqTabId::Params,
        }
    }
}
