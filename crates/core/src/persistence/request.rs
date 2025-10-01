use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::assertions::Assertions;
use crate::http::request::{Auth, Method, Request, RequestBody};
use crate::http::{KeyFile, KeyFileList};
use crate::persistence::Version;

use super::{EncodedKeyFile, EncodedKeyValue, decode_key_values, encode_key_values};

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
pub struct EncodedRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: Version,
    pub method: EncodedMethod,
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub queries: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<EncodedKeyValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<EncodedAuthType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<EncodedRequestBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_request: Option<String>,
    #[serde(default, skip_serializing_if = "Assertions::is_empty")]
    pub assertions: Assertions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodedRequestBody {
    Form(Vec<EncodedKeyValue>),
    Multipart {
        params: Vec<EncodedKeyValue>,
        files: Vec<EncodedKeyFile>,
    },
    Json(String),
    XML(String),
    Text(String),
    File(Option<PathBuf>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EncodedAuthType {
    Basic { username: String, password: String },
    Bearer { token: String },
}

pub fn encode_request(req: Request) -> EncodedRequest {
    let Request {
        method,
        url,
        headers,
        body,
        query_params,
        path_params,
        auth,
        description,
        assertions,
        pre_request,
    } = req;

    let encoded_method = match method {
        Method::GET => EncodedMethod::GET,
        Method::POST => EncodedMethod::POST,
        Method::PUT => EncodedMethod::PUT,
        Method::DELETE => EncodedMethod::DELETE,
        Method::PATCH => EncodedMethod::PATCH,
        Method::HEAD => EncodedMethod::HEAD,
        Method::OPTIONS => EncodedMethod::OPTIONS,
        Method::CONNECT => EncodedMethod::CONNECT,
        Method::TRACE => EncodedMethod::TRACE,
    };

    EncodedRequest {
        description,
        version: Version::V1,
        method: encoded_method,
        url,
        params: encode_key_values(path_params),
        queries: encode_key_values(query_params),
        headers: encode_key_values(headers),
        auth: encode_auth(auth),
        body: encode_body(body),
        pre_request,
        assertions,
    }
}

fn encode_body(body: RequestBody) -> Option<EncodedRequestBody> {
    match body {
        RequestBody::None => None,
        RequestBody::Form(form) => Some(EncodedRequestBody::Form(encode_key_values(form))),
        RequestBody::Json(data) => Some(EncodedRequestBody::Json(data)),
        RequestBody::XML(data) => Some(EncodedRequestBody::XML(data)),
        RequestBody::Text(data) => Some(EncodedRequestBody::Text(data)),
        RequestBody::File(path) => Some(EncodedRequestBody::File(path)),
        RequestBody::Multipart { params, files } => Some(EncodedRequestBody::Multipart {
            params: encode_key_values(params),
            files: encode_key_files(files),
        }),
    }
}

fn encode_key_files(files: KeyFileList) -> Vec<EncodedKeyFile> {
    files
        .into_iter()
        .filter(|v| !v.name.is_empty())
        .map(|v| EncodedKeyFile {
            name: v.name,
            path: v.path,
            disabled: v.disabled,
        })
        .collect()
}

fn encode_auth(auth: Auth) -> Option<EncodedAuthType> {
    match auth {
        Auth::None => None,
        Auth::Basic { username, password } => Some(EncodedAuthType::Basic { username, password }),
        Auth::Bearer { token } => Some(EncodedAuthType::Bearer { token }),
    }
}

fn decode_key_files(files: Vec<EncodedKeyFile>) -> KeyFileList {
    let mut list = Vec::new();
    for v in files {
        list.push(KeyFile {
            name: v.name,
            path: v.path,
            disabled: v.disabled,
        });
    }

    KeyFileList::from(list)
}

fn decode_body(body: Option<EncodedRequestBody>) -> Option<RequestBody> {
    let body = body?;
    let decode = match body {
        EncodedRequestBody::Form(form) => RequestBody::Form(decode_key_values(form)),
        EncodedRequestBody::Json(data) => RequestBody::Json(data),
        EncodedRequestBody::XML(data) => RequestBody::XML(data),
        EncodedRequestBody::Text(data) => RequestBody::Text(data),
        EncodedRequestBody::File(path) => RequestBody::File(path),
        EncodedRequestBody::Multipart { params, files } => RequestBody::Multipart {
            params: decode_key_values(params),
            files: decode_key_files(files),
        },
    };

    Some(decode)
}

fn decode_auth(auth: Option<EncodedAuthType>) -> Auth {
    match auth {
        None => Auth::None,
        Some(EncodedAuthType::Basic { username, password }) => Auth::Basic { username, password },
        Some(EncodedAuthType::Bearer { token }) => Auth::Bearer { token },
    }
}

fn decode_request(req: EncodedRequest) -> Request {
    let EncodedRequest {
        description,
        method,
        url,
        headers,
        body,
        queries: query,
        params: path_params,
        auth,
        pre_request,
        assertions,
        ..
    } = req;

    Request {
        method: method.into(),
        url,
        headers: decode_key_values(headers),
        body: decode_body(body).unwrap_or(RequestBody::None),
        query_params: decode_key_values(query),
        path_params: decode_key_values(path_params),
        auth: decode_auth(auth),
        description,
        assertions,
        pre_request,
    }
}

pub async fn read_request(path: &PathBuf) -> anyhow::Result<Request> {
    let request = load_from_file(path).await?;
    Ok(decode_request(request))
}

pub async fn save_req_to_file(path: PathBuf, req: EncodedRequest) -> Result<(), anyhow::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let file = fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let encoded = toml::to_string_pretty(&req)?;

    writer.write_all(encoded.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_from_file(path: &PathBuf) -> anyhow::Result<EncodedRequest> {
    let file = fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;
    let decoded: EncodedRequest = toml::from_str(&buffer)?;

    Ok(decoded)
}
