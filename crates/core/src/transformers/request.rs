use std::path::PathBuf;

use anyhow::Context;
use mime::{APPLICATION_JSON, TEXT_PLAIN, TEXT_XML};
use mime_guess::{mime, Mime};
use regex::Regex;
use reqwest::multipart::Part;
use reqwest::RequestBuilder;
use reqwest::{header::CONTENT_TYPE, multipart::Form};
use tokio::fs::File;

use crate::http::{
    environment::Environment,
    request::{Auth, Method, Request, RequestBody},
    KeyFileList, KeyValList, KeyValue,
};

fn param_enabled(param: &KeyValue) -> bool {
    !param.disabled && !param.name.is_empty()
}

fn enabled_params(params: KeyValList, env: Option<&Environment>) -> Vec<(String, String)> {
    params
        .into_iter()
        .filter(param_enabled)
        .map(|param| (param.name, replace_env_vars(&param.value, env)))
        .collect()
}

fn enabled_files(files: KeyFileList, _env: Option<&Environment>) -> Vec<(String, PathBuf)> {
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
    env: Option<&Environment>,
) -> RequestBuilder {
    let iter = headers.into_iter().filter(param_enabled);
    for header in iter {
        builder = builder.header(header.name, replace_env_vars(&header.value, env));
    }
    builder
}

fn req_params(
    builder: RequestBuilder,
    params: KeyValList,
    env: Option<&Environment>,
) -> RequestBuilder {
    let params = enabled_params(params, env);
    builder.query(&params)
}
pub async fn transform_request(
    client: reqwest::Client,
    req: Request,
    env: Option<Environment>,
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

    let env = env.as_ref();

    let url = replace_path_params(url, path_params, env);
    let mut builder = client.request(req_method(method), url);

    builder = req_headers(builder, headers, env);
    builder = req_params(builder, query_params, env);
    builder = req_auth(builder, auth, env);
    builder = req_body(builder, body, env).await;

    builder.build().context("Failed to build request")
}

fn replace_env_vars(source: &str, env: Option<&Environment>) -> String {
    let Some(env) = env else {
        return source.to_string();
    };
    let replaced = Regex::new(r"\{\{([a-zA-Z0-9]+)\}\}").unwrap().replace_all(
        source,
        |cap: &regex::Captures| -> String {
            let name = &cap[1];
            env.get(name).unwrap_or(name).to_string()
        },
    );
    replaced.to_string()
}

fn replace_path_params(url: String, params: KeyValList, env: Option<&Environment>) -> String {
    let url = replace_env_vars(&url, env);
    let replaced = Regex::new(r":([a-zA-Z0-9]+)").unwrap().replace_all(
        &url,
        |cap: &regex::Captures| -> String {
            let name = &cap[1];
            let value = params
                .iter()
                .find(|param| param.name == name)
                .map(|param| replace_env_vars(&param.value, env))
                .unwrap_or_else(|| name.to_owned());
            value
        },
    );
    replaced.to_string()
}

async fn req_body(
    builder: RequestBuilder,
    body: RequestBody,
    env: Option<&Environment>,
) -> RequestBuilder {
    let body_header = |builder: RequestBuilder, data, content_type: Mime| {
        builder
            .body(replace_env_vars(data, env))
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
    env: Option<&Environment>,
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
        .expect("Failed to open file");
    (content_type, file)
}

fn req_auth(builder: RequestBuilder, auth: Auth, env: Option<&Environment>) -> RequestBuilder {
    match auth {
        Auth::None => builder,
        Auth::Basic { username, password } => {
            let username = replace_env_vars(&username, env);
            let password = replace_env_vars(&password, env);
            builder.basic_auth(username, Some(password))
        }
        Auth::Bearer { token } => {
            let token = replace_env_vars(&token, env);
            builder.bearer_auth(token)
        }
    }
}
