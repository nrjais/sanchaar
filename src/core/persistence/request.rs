use crate::components::{KeyValList, KeyValue};
use crate::core::persistence::fs::load_from_file;
use crate::core::persistence::Version;
use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::ops::Not;
use std::path::PathBuf;

use crate::state::request::{Method, Request, RequestBody};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EncodedMethod {
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

impl From<Method> for EncodedMethod {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => EncodedMethod::GET,
            Method::POST => EncodedMethod::POST,
            Method::PUT => EncodedMethod::PUT,
            Method::DELETE => EncodedMethod::DELETE,
            Method::PATCH => EncodedMethod::PATCH,
            Method::HEAD => EncodedMethod::HEAD,
            Method::OPTIONS => EncodedMethod::OPTIONS,
            Method::CONNECT => EncodedMethod::CONNECT,
            Method::TRACE => EncodedMethod::TRACE,
        }
    }
}

impl From<EncodedMethod> for Method {
    fn from(val: EncodedMethod) -> Self {
        match val {
            EncodedMethod::GET => Method::GET,
            EncodedMethod::POST => Method::POST,
            EncodedMethod::PUT => Method::PUT,
            EncodedMethod::DELETE => Method::DELETE,
            EncodedMethod::PATCH => Method::PATCH,
            EncodedMethod::HEAD => Method::HEAD,
            EncodedMethod::OPTIONS => Method::OPTIONS,
            EncodedMethod::CONNECT => Method::CONNECT,
            EncodedMethod::TRACE => Method::TRACE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedKeyValue {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Not::not")]
    pub disabled: bool,
}

impl From<&KeyValue> for EncodedKeyValue {
    fn from(value: &KeyValue) -> Self {
        EncodedKeyValue {
            name: value.name.clone(),
            value: value.value.clone(),
            disabled: value.disabled,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedRequest {
    pub http: HttpRequest,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: EncodedMethod,
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub query: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_params: Vec<EncodedKeyValue>,
}

fn encode_key_values(kv: &KeyValList) -> Vec<EncodedKeyValue> {
    kv.values()
        .iter()
        .filter(|v| !v.name.is_empty())
        .map(|v| v.into())
        .collect()
}

pub fn encode_request(req: &Request) -> EncodedRequest {
    EncodedRequest {
        http: HttpRequest {
            method: req.method.into(),
            url: req.url.clone(),
            headers: encode_key_values(&req.headers),
            query: encode_key_values(&req.query_params),
            path_params: encode_key_values(&req.path_params),
        },
        description: req.description.clone(),
        version: Version::V1,
    }
}

fn decode_key_values(kv: &[EncodedKeyValue], fixed: bool) -> KeyValList {
    let mut list = Vec::new();
    for v in kv {
        list.push(KeyValue {
            name: v.name.clone(),
            value: v.value.clone(),
            disabled: v.disabled,
        });
    }

    KeyValList::from(list, fixed)
}

fn decode_request(req: &EncodedRequest) -> Request {
    Request {
        method: req.http.method.into(),
        url: req.http.url.clone(),
        headers: decode_key_values(&req.http.headers, false),
        body: RequestBody::None,
        query_params: decode_key_values(&req.http.query, false),
        path_params: decode_key_values(&req.http.path_params, true),
        description: req.description.clone(),
    }
}

pub async fn read_request(path: PathBuf) -> anyhow::Result<Request> {
    let enc_req = load_from_file(&path)
        .map_err(|_| anyhow::format_err!("Failed to read request"))
        .await?;
    Ok(decode_request(&enc_req))
}
