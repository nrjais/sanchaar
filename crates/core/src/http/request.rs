use std::path::PathBuf;
use strum::{Display, EnumString, VariantArray};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyValue {
    pub disabled: bool,
    pub name: String,
    pub value: String,
}

pub type KeyValList = Vec<KeyValue>;

#[derive(Debug, Clone)]
pub enum RequestBody {
    Form(KeyValList),
    Json(String),
    XML(String),
    Text(String),
    File(Option<PathBuf>),
    None,
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
            path_params: KeyValList::new(),
        }
    }
}
