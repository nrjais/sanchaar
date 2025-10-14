use std::path::PathBuf;

use anyhow::Context;
use jsonwebtoken::{EncodingKey, Header, encode};
use mime::{APPLICATION_JSON, TEXT_PLAIN, TEXT_XML};
use mime_guess::{Mime, mime};
use reqwest::multipart::Part;
use reqwest::{RequestBuilder, Url};
use reqwest::{header::CONTENT_TYPE, multipart::Form};
use serde_json::Value;
use tokio::fs::File;

use crate::http::environment::EnvironmentChain;
use crate::http::{
    KeyFileList, KeyValList, KeyValue,
    request::{Auth, AuthIn, Method, Request, RequestBody},
};
use crate::{APP_NAME, APP_VERSION};

fn param_enabled(param: &KeyValue) -> bool {
    !param.disabled && !param.name.is_empty()
}

fn enabled_params(params: KeyValList, env: &EnvironmentChain) -> Vec<(String, String)> {
    params
        .into_iter()
        .filter(param_enabled)
        .map(|param| (param.name, env.replace(&param.value)))
        .collect()
}

fn enabled_files(files: KeyFileList, _env: &EnvironmentChain) -> Vec<(String, PathBuf)> {
    files
        .into_iter()
        .filter(|file| !file.name.is_empty())
        .filter_map(|file| file.path.map(|path| (file.name, path)))
        .collect()
}

fn req_method(method: Method) -> reqwest::Method {
    match method {
        Method::GET => reqwest::Method::GET,
        Method::POST => reqwest::Method::POST,
        Method::PUT => reqwest::Method::PUT,
        Method::DELETE => reqwest::Method::DELETE,
        Method::PATCH => reqwest::Method::PATCH,
        Method::HEAD => reqwest::Method::HEAD,
        Method::OPTIONS => reqwest::Method::OPTIONS,
        Method::CONNECT => reqwest::Method::CONNECT,
        Method::TRACE => reqwest::Method::TRACE,
    }
}

fn req_headers(
    mut builder: RequestBuilder,
    headers: KeyValList,
    env: &EnvironmentChain,
) -> RequestBuilder {
    let iter = headers.into_iter().filter(param_enabled);
    let mut has_user_agent = false;
    for header in iter {
        if header.name.to_lowercase() == "user-agent" {
            has_user_agent = true;
        }
        builder = builder.header(header.name, env.replace(&header.value));
    }

    if !has_user_agent {
        let user_agent = format!("{}/{}", APP_NAME, APP_VERSION);
        builder = builder.header("User-Agent", user_agent);
    }

    builder
}

fn req_params(
    builder: RequestBuilder,
    params: KeyValList,
    env: &EnvironmentChain,
) -> RequestBuilder {
    let params = enabled_params(params, env);
    builder.query(&params)
}

pub async fn transform_request(
    client: reqwest::Client,
    req: Request,
    env: EnvironmentChain,
) -> anyhow::Result<reqwest::Request> {
    let Request {
        method,
        url,
        path_params,
        headers,
        query_params,
        body,
        auth,
        ..
    } = req;

    let env = &env;

    let url = process_url(url, env, path_params)?;
    let mut builder = client.request(req_method(method), url);

    builder = req_headers(builder, headers, env);
    builder = req_params(builder, query_params, env);
    builder = req_auth(builder, auth, env);
    builder = req_body(builder, body, env).await;

    builder.build().context("Failed to build request")
}

fn process_url(
    url: String,
    env: &EnvironmentChain,
    path_params: KeyValList,
) -> Result<Url, anyhow::Error> {
    let url = env.replace(&url);
    let normalized_url = normalize_url(&url);
    let url = Url::parse(&normalized_url).context("Failed to parse URL")?;
    let url = replace_path_params(url, path_params, env);
    Ok(url)
}

fn normalize_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }

    if url.starts_with("localhost") || url.starts_with("127.0.0.1") {
        format!("http://{}", url)
    } else {
        format!("https://{}", url)
    }
}

fn replace_path_params(mut url: Url, params: KeyValList, env: &EnvironmentChain) -> Url {
    let Some(segs) = url.path_segments() else {
        return url;
    };
    let mut buffer = String::new();

    for seg in segs {
        buffer.push('/');
        if let Some(name) = seg.strip_prefix(':') {
            let value = params
                .iter()
                .rev()
                .find(|param| param.name == name)
                .map(|param| env.replace(&param.value))
                .unwrap_or_else(|| name.to_owned());
            buffer.push_str(&value);
        } else {
            buffer.push_str(seg);
        }
    }

    url.set_path(&buffer);
    url
}

async fn req_body(
    builder: RequestBuilder,
    body: RequestBody,
    env: &EnvironmentChain,
) -> RequestBuilder {
    let body_header = |builder: RequestBuilder, data, content_type: Mime| {
        builder
            .body(env.replace(data))
            .header(CONTENT_TYPE, content_type.as_ref())
    };

    match body {
        RequestBody::Text(text) => body_header(builder, &text, TEXT_PLAIN),
        RequestBody::Json(json) => body_header(builder, &json, APPLICATION_JSON),
        RequestBody::XML(xml) => body_header(builder, &xml, TEXT_XML),
        RequestBody::Form(form) => builder.form(&enabled_params(form, env)),
        RequestBody::File(Some(file)) => file_body(file, builder).await,
        RequestBody::None | RequestBody::File(None) => builder,
        RequestBody::Multipart { params, files } => multipart(builder, params, files, env).await,
    }
}

async fn multipart(
    builder: RequestBuilder,
    params: KeyValList,
    files: KeyFileList,
    env: &EnvironmentChain,
) -> RequestBuilder {
    let params = enabled_params(params, env);
    let files = enabled_files(files, env);
    let mut form = Form::new();

    for (name, value) in params {
        form = form.text(name, value);
    }

    for (name, path) in files {
        let (content_type, file) = open_file(&path).await;
        let filename = path.file_name().unwrap().to_string_lossy().to_string();

        let part = Part::stream(file)
            .file_name(filename)
            .mime_str(&content_type)
            .unwrap();

        form = form.part(name, part);
    }

    builder.multipart(form)
}

async fn file_body(file: PathBuf, builder: RequestBuilder) -> RequestBuilder {
    let (content_type, file) = open_file(&file).await;
    builder.body(file).header(CONTENT_TYPE, content_type)
}

// Files are only sent with non GET requests
async fn open_file(file: &PathBuf) -> (String, File) {
    let content_type = mime_guess::from_path(file)
        .first_or_octet_stream()
        .to_string();

    let file = tokio::fs::OpenOptions::new()
        .read(true)
        .open(file)
        .await
        .expect("Failed to open file for request body");
    (content_type, file)
}

fn req_auth(builder: RequestBuilder, auth: Auth, env: &EnvironmentChain) -> RequestBuilder {
    match auth {
        Auth::None => builder,
        Auth::Basic { username, password } => {
            let username = env.replace(&username);
            let password = env.replace(&password);
            builder.basic_auth(username, Some(password))
        }
        Auth::Bearer { token } => {
            let token = env.replace(&token);
            builder.bearer_auth(token)
        }
        Auth::APIKey { key, value, add_to } => {
            let key = env.replace(&key);
            let value = env.replace(&value);

            match add_to {
                AuthIn::Header => builder.header(key, value),
                AuthIn::Query => builder.query(&[(key, value)]),
            }
        }
        Auth::JWTBearer {
            algorithm,
            secret,
            payload,
            add_to,
        } => {
            let secret = env.replace(&secret);
            let payload = env.replace(&payload);

            let claims: Value =
                serde_json::from_str(&payload).unwrap_or(Value::Object(Default::default()));

            let token = encode(
                &Header::new((&algorithm).into()),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .unwrap_or_else(|_| String::new());

            match add_to {
                AuthIn::Header => builder.bearer_auth(token),
                AuthIn::Query => builder.query(&[("token", token)]),
            }
        }
    }
}
