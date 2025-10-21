use jsonwebtoken::Algorithm;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JwtAlgorithm {
    #[default]
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    PS256,
    PS384,
    PS512,
    EdDSA,
}

impl From<&JwtAlgorithm> for Algorithm {
    fn from(val: &JwtAlgorithm) -> Self {
        match val {
            JwtAlgorithm::HS256 => Algorithm::HS256,
            JwtAlgorithm::HS384 => Algorithm::HS384,
            JwtAlgorithm::HS512 => Algorithm::HS512,
            JwtAlgorithm::RS256 => Algorithm::RS256,
            JwtAlgorithm::RS384 => Algorithm::RS384,
            JwtAlgorithm::RS512 => Algorithm::RS512,
            JwtAlgorithm::ES256 => Algorithm::ES256,
            JwtAlgorithm::ES384 => Algorithm::ES384,
            JwtAlgorithm::PS256 => Algorithm::PS256,
            JwtAlgorithm::PS384 => Algorithm::PS384,
            JwtAlgorithm::PS512 => Algorithm::PS512,
            JwtAlgorithm::EdDSA => Algorithm::EdDSA,
        }
    }
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
        algorithm: JwtAlgorithm,
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
    pub post_request: Option<String>,
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
            post_request: None,
        }
    }
}
