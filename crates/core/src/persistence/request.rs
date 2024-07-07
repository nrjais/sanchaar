use std::path::PathBuf;

use anyhow::Context;
use hcl::expr::{Heredoc, TemplateExpr};
use hcl::structure::BodyBuilder;
use hcl::{Block, Body, Expression, Value};
use iced::futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::http::request::{Auth, Method, Request, RequestBody};
use crate::http::{KeyFile, KeyFileList, KeyValList, KeyValue};
use crate::persistence::Version;

use super::{to_hcl_pretty, EncodedKeyFile, EncodedKeyValue, HCL_EXTENSION};

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
pub struct EncodedRequest {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    pub version: Version,
    pub http: HttpRequest,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: EncodedMethod,
    pub url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path_params: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub query: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<EncodedAuthType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<EncodedKeyValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<EncodedRequestBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_request: Option<String>,
}

pub fn encode_request(req: Request) -> Body {
    let Request {
        method,
        url,
        headers,
        body,
        query_params,
        path_params,
        auth,
        description,
        pre_request,
    } = req;

    let mut builder = Body::builder()
        .add_block(
            Block::builder("sanchaar")
                .add_attribute(("version", Version::V1.to_string()))
                .add_attribute(("description", description))
                .build(),
        )
        .add_attribute(("method", method.to_string()))
        .add_attribute(("url", url));

    builder = add_auth_block(builder, auth);
    builder = add_kv_block(builder, "params", path_params);
    builder = add_kv_block(builder, "queries", query_params);
    builder = add_kv_block(builder, "headers", headers);
    builder = add_body_block(builder, body);

    if let Some(pre_request) = pre_request {
        builder = builder.add_attribute(("pre_request", pre_request));
    }

    builder.build()
}

fn add_body_block(builder: BodyBuilder, body: RequestBody) -> BodyBuilder {
    let block = Block::builder("body");
    let block = match body {
        RequestBody::None => return builder,
        RequestBody::Form(form) => block.add_block(build_block("urlencoded", form)),
        RequestBody::Json(data) => block.add_attribute(("json", multiline_text(data))),
        RequestBody::XML(data) => block.add_attribute(("xml", multiline_text(data))),
        RequestBody::Text(data) => block.add_attribute(("text", multiline_text(data))),
        RequestBody::File(path) => block.add_attribute(("file", path_expr(path))),
        RequestBody::Multipart { params, files } => block
            .add_block(build_block("form", params))
            .add_block(build_block_files("files", files)),
    };

    builder.add_block(block.build())
}

fn multiline_text(data: String) -> Expression {
    // Find the longest running sequence of only underscores in line
    let max_running = data
        .chars()
        .fold(0, |acc, c| if c == '_' { acc + 1 } else { 0 });

    let delimiter = "_".repeat(max_running + 1);

    let template = TemplateExpr::Heredoc(Heredoc::new(delimiter.into(), data));
    Expression::TemplateExpr(Box::new(template))
}

fn path_expr(path: Option<PathBuf>) -> Expression {
    path.unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .into()
}

fn add_auth_block(body: BodyBuilder, auth: Auth) -> BodyBuilder {
    let block = match auth {
        Auth::None => return body,
        Auth::Basic { username, password } => Block::builder("auth")
            .add_label("basic")
            .add_attribute(("username", username))
            .add_attribute(("password", password))
            .build(),
        Auth::Bearer { token } => Block::builder("auth")
            .add_label("bearer")
            .add_attribute(("token", token))
            .build(),
    };

    body.add_block(block)
}

fn add_kv_block(body: BodyBuilder, name: &'static str, kv: KeyValList) -> BodyBuilder {
    let mut body = body;
    if !kv.is_empty() {
        body = body.add_block(build_block(name, kv));
    }
    body
}

fn build_block(name: &str, kv: impl IntoIterator<Item = KeyValue>) -> Block {
    let mut block = Block::builder(name);
    for param in kv {
        let value = Expression::String(param.value);
        let value = match param.disabled {
            true => Expression::Object({
                let mut obj = hcl::Object::new();
                obj.insert("disabled".into(), Expression::Bool(true));
                obj.insert("value".into(), value);
                obj
            }),
            false => value,
        };

        block = block.add_attribute((param.name, value));
    }
    block.build()
}

fn build_block_files(name: &str, kv: impl IntoIterator<Item = KeyFile>) -> Block {
    let mut block = Block::builder(name);
    for param in kv {
        let path = path_expr(param.path);
        let value = match param.disabled {
            true => Expression::Object({
                let mut obj = hcl::Object::new();
                obj.insert("disabled".into(), Expression::Bool(true));
                obj.insert("path".into(), path);
                obj
            }),
            false => path,
        };
        block = block.add_attribute((param.name, value));
    }

    block.build()
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

    KeyValList::from(list)
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
        http, description, ..
    } = req;
    let HttpRequest {
        method,
        url,
        headers,
        body,
        query,
        path_params,
        auth,
        pre_request,
    } = http;

    Request {
        method: method.into(),
        url,
        headers: decode_key_values(headers),
        body: decode_body(body).unwrap_or(RequestBody::None),
        query_params: decode_key_values(query),
        path_params: decode_key_values(path_params),
        auth: decode_auth(auth),
        description,
        pre_request,
    }
}

pub async fn read_request(path: PathBuf) -> anyhow::Result<Request> {
    let enc_req = load_from_file(&path)
        .map_err(|_| anyhow::format_err!("Failed to read request"))
        .await?;
    Ok(decode_request(enc_req))
}

pub async fn save_req_to_file(path: PathBuf, req: hcl::Body) -> Result<(), anyhow::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let path = path.with_extension(HCL_EXTENSION);
    let file = fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let encoded = to_hcl_pretty(&req)?;

    writer.write_all(encoded.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_from_file(path: &PathBuf) -> Result<EncodedRequest, Box<dyn std::error::Error>> {
    let file = fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;
    let decoded: EncodedRequest = toml::from_str(&buffer)?;

    Ok(decoded)
}
