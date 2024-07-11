use std::path::PathBuf;

use hcl::expr::{Heredoc, TemplateExpr};
use hcl::structure::BodyBuilder;
use hcl::value::to_value;
use hcl::{Block, Body, Expression};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::assertions::{self, Assertions};
use crate::http::request::{Auth, Method, Request, RequestBody};
use crate::http::{KeyFile, KeyFileList, KeyValList, KeyValue};
use crate::persistence::Version;

use super::{EncodedKeyFile, EncodedKeyValue};

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

#[derive(Debug, Deserialize)]
pub struct EncodedRequest {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: Version,
    pub method: EncodedMethod,
    pub url: String,
    #[serde(default)]
    pub params: Vec<EncodedKeyValue>,
    #[serde(default)]
    pub queries: Vec<EncodedKeyValue>,
    #[serde(default)]
    pub headers: Vec<EncodedKeyValue>,
    pub auth: Option<EncodedAuthType>,
    pub body: Option<EncodedRequestBody>,
    pub pre_request: Option<String>,
    #[serde(default)]
    pub assertions: Assertions,
}

#[derive(Debug, Deserialize)]
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

pub fn encode_request(req: Request) -> hcl::Result<Body> {
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

    let mut builder = Body::builder()
        .add_attribute(("version", Version::V1.to_string()))
        .add_attribute(("description", description))
        .add_attribute(("url", url))
        .add_attribute(("method", method.to_string()));

    builder = add_auth_block(builder, auth)?;
    builder = add_kv_block(builder, "params", path_params)?;
    builder = add_kv_block(builder, "queries", query_params)?;
    builder = add_kv_block(builder, "headers", headers)?;
    builder = add_body_block(builder, body)?;
    builder = assertions::encode(builder, assertions);

    if let Some(pre_request) = pre_request {
        builder = builder.add_attribute(("pre_request", pre_request));
    }

    Ok(builder.build())
}

fn add_body_block(builder: BodyBuilder, body: RequestBody) -> hcl::Result<BodyBuilder> {
    let block = Block::builder("body");
    let block = match body {
        RequestBody::None => return Ok(builder),
        RequestBody::Form(form) => {
            block.add_attribute(("form", to_value(encode_key_values(form))?))
        }
        RequestBody::Json(data) => block.add_attribute(("json", multiline_text(data))),
        RequestBody::XML(data) => block.add_attribute(("xml", multiline_text(data))),
        RequestBody::Text(data) => block.add_attribute(("text", multiline_text(data))),
        RequestBody::File(path) => block.add_attribute(("file", path_expr(path))),
        RequestBody::Multipart { params, files } => block.add_block(
            Block::builder("multipart")
                .add_attribute(("params", to_value(encode_key_values(params))?))
                .add_attribute(("files", to_value(encode_key_files(files))?))
                .build(),
        ),
    };

    Ok(builder.add_block(block.build()))
}

fn multiline_text(data: String) -> Expression {
    if data.len() < 120 && !data.contains('\n') {
        return data.into();
    }

    // Find the longest running sequence of only underscores in line
    let max_running = data
        .chars()
        .fold(0, |acc, c| if c == '_' { acc + 1 } else { 0 });

    let delimiter = "_".repeat(max_running + 2);

    let template = TemplateExpr::Heredoc(Heredoc::new(delimiter.into(), data));
    Expression::TemplateExpr(Box::new(template))
}

fn path_expr(path: Option<PathBuf>) -> Expression {
    path.unwrap_or_default()
        .to_string_lossy()
        .to_string()
        .into()
}

fn encode_key_values(kv: KeyValList) -> Vec<EncodedKeyValue> {
    kv.into_iter().map(|v| v.into()).collect()
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
    let en = match auth {
        Auth::None => return None,
        Auth::Basic { username, password } => EncodedAuthType::Basic { username, password },
        Auth::Bearer { token } => EncodedAuthType::Bearer { token },
    };
    Some(en)
}

fn add_auth_block(body: BodyBuilder, auth: Auth) -> hcl::Result<BodyBuilder> {
    let encoded = encode_auth(auth);
    match encoded {
        Some(auth) => Ok(body.add_attribute(("auth", to_value(auth)?))),
        None => Ok(body),
    }
}

fn add_kv_block(body: BodyBuilder, name: &'static str, kv: KeyValList) -> hcl::Result<BodyBuilder> {
    let mut body = body;
    if !kv.is_empty() {
        body = body.add_attribute((name, to_value(encode_key_values(kv))?));
    }
    Ok(body)
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

pub async fn read_request(path: PathBuf) -> anyhow::Result<Request> {
    let request = load_from_file(&path).await?;
    Ok(decode_request(request))
}

pub async fn save_req_to_file(path: PathBuf, req: hcl::Body) -> Result<(), anyhow::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let file = fs::File::create(path).await?;
    let mut writer = BufWriter::new(file);
    let encoded = hcl::to_string(&req)?;

    writer.write_all(encoded.as_bytes()).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn load_from_file(path: &PathBuf) -> anyhow::Result<EncodedRequest> {
    let file = fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut buffer = String::new();

    reader.read_to_string(&mut buffer).await?;
    let decoded: EncodedRequest = hcl::from_str(&buffer)?;

    Ok(decoded)
}
