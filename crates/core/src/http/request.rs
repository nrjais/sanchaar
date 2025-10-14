use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{Display, EnumString, VariantArray};

use crate::assertions::Assertions;

use super::{KeyFileList, KeyValList};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthIn {
    #[default]
    Query,
    Header,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Auth {
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    APIKey {
        key: String,
        value: String,
        add_to: AuthIn,
    },
    JWTBearer {
        secret: String,
        payload: String,
        add_to: AuthIn,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, VariantArray, Display, Default)]
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

impl Request {
    pub fn extend_headers(&mut self, headers: &KeyValList) {
        self.headers.extend(headers.clone());
    }
}

impl Default for Request {
    fn default() -> Self {
        Self {
            description: "Http request".to_string(),
            method: Method::GET,
            url: "https://echo.sanchaar.app".to_string(),
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
