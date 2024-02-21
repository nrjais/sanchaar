use iced::widget::text_editor;
use iced_aw::split;
use std::path::PathBuf;
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

    pub fn all_variants() -> Vec<&'static str> {
        Vec::from(["URL Encoded", "Json", "XML", "Text", "File", "None"])
    }
}

#[derive(Debug, Default)]
pub struct RequestPane {
    pub url: String,
    pub method: Method,
    pub headers: KeyValList,
    pub body: RequestRawBody,
    pub query_params: KeyValList,
    pub split_axis: split::Axis,
    pub split_pos: Option<u16>,
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
    pub method: Method,
    pub url: String,
    pub headers: KeyValList,
    pub body: RequestBody,
    pub query_params: KeyValList,
}

impl RequestPane {
    pub(crate) fn new() -> RequestPane {
        RequestPane {
            url: "https://echo.nrjais.com".to_string(),
            ..Default::default()
        }
    }

    pub fn to_request(&self) -> Request {
        Request {
            method: self.method,
            url: self.url.clone(),
            headers: self.headers.clone(),
            body: self.body.to_request_body(),
            query_params: self.query_params.clone(),
        }
    }
}
