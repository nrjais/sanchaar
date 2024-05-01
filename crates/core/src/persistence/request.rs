use std::ops::Not;
use std::path::PathBuf;

use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::http::request::{KeyValList, KeyValue, Method, Request, RequestBody};
use crate::persistence::Version;

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

impl From<KeyValue> for EncodedKeyValue {
    fn from(value: KeyValue) -> Self {
        EncodedKeyValue {
            name: value.name,
            value: value.value,
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
pub enum EncodedRequestBody {
    Form(Vec<EncodedKeyValue>),
    Json(String),
    XML(String),
    Text(String),
    File(Option<PathBuf>),
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<EncodedRequestBody>,
}

fn encode_key_values(kv: KeyValList) -> Vec<EncodedKeyValue> {
    kv.into_iter()
        .filter(|v| !v.name.is_empty())
        .map(|v| v.into())
        .collect()
}

fn encode_body(body: RequestBody) -> Option<EncodedRequestBody> {
    match body {
        RequestBody::Form(form) => Some(EncodedRequestBody::Form(encode_key_values(form))),
        RequestBody::Json(data) => Some(EncodedRequestBody::Json(data)),
        RequestBody::XML(data) => Some(EncodedRequestBody::XML(data)),
        RequestBody::Text(data) => Some(EncodedRequestBody::Text(data)),
        RequestBody::File(path) => Some(EncodedRequestBody::File(path)),
        RequestBody::None => None,
    }
}

pub fn encode_request(req: Request) -> EncodedRequest {
    let Request {
        method,
        url,
        headers,
        body,
        query_params,
        path_params,
        description,
    } = req;

    EncodedRequest {
        http: HttpRequest {
            method: method.into(),
            url,
            headers: encode_key_values(headers),
            query: encode_key_values(query_params),
            path_params: encode_key_values(path_params),
            body: encode_body(body),
        },
        description,
        version: Version::V1,
    }
}

fn decode_key_values(kv: Vec<EncodedKeyValue>) -> KeyValList {
    let mut list = Vec::new();
    for v in kv {
        list.push(KeyValue {
            name: v.name,
            value: v.value,
            disabled: v.disabled,
        });
    }

    list
}

fn decode_body(body: Option<EncodedRequestBody>) -> Option<RequestBody> {
    let body = body?;
    let decode = match body {
        EncodedRequestBody::Form(form) => RequestBody::Form(decode_key_values(form)),
        EncodedRequestBody::Json(data) => RequestBody::Json(data),
        EncodedRequestBody::XML(data) => RequestBody::XML(data),
        EncodedRequestBody::Text(data) => RequestBody::Text(data),
        EncodedRequestBody::File(path) => RequestBody::File(path),
    };

    Some(decode)
}

fn decode_request(req: EncodedRequest) -> Request {
    let EncodedRequest {
        http, description, ..
    } = req;
    let HttpRequest {
        method,
        url,
        headers,
        body,
        query,
        path_params,
    } = http;

    Request {
        method: method.into(),
        url,
        headers: decode_key_values(headers),
        body: decode_body(body).unwrap_or(RequestBody::None),
        query_params: decode_key_values(query),
        path_params: decode_key_values(path_params),
        description,
    }
}

pub async fn read_request(path: PathBuf) -> anyhow::Result<Request> {
    let enc_req = load_from_file(&path)
        .map_err(|_| anyhow::format_err!("Failed to read request"))
        .await?;
    Ok(decode_request(enc_req))
}

pub async fn save_req_to_file(path: PathBuf, req: EncodedRequest) -> Result<(), anyhow::Error> {
    let file = tokio::fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let encoded = toml::to_string_pretty(&req)?;

    writer.write_all(encoded.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_from_file(path: &PathBuf) -> Result<EncodedRequest, Box<dyn std::error::Error>> {
    let file = tokio::fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;
    let decoded: EncodedRequest = toml::from_str(&buffer)?;

    Ok(decoded)
}
