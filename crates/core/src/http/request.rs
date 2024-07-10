use std::path::PathBuf;
use strum::{Display, EnumString, VariantArray};

use super::{assertions::Assertions, KeyFileList, KeyValList};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestBody {
    Multipart {
        params: KeyValList,
        files: KeyFileList,
    },
    Form(KeyValList),
    Json(String),
    XML(String),
    Text(String),
    File(Option<PathBuf>),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Auth {
    None,
    Basic { username: String, password: String },
    Bearer { token: String },
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

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub description: String,
    pub method: Method,
    pub url: String,
    pub headers: KeyValList,
    pub body: RequestBody,
    pub query_params: KeyValList,
    pub path_params: KeyValList,
    pub auth: Auth,
    pub assertions: Assertions,
    pub pre_request: Option<String>,
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
            path_params: KeyValList::new(),
            auth: Auth::None,
            assertions: Assertions::default(),
            pre_request: None,
        }
    }
}
