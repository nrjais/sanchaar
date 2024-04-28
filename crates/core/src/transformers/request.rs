use anyhow::Context;
use mime::{APPLICATION_JSON, TEXT_PLAIN, TEXT_XML};
use mime_guess::mime;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;

use crate::http::request::{KeyValList, KeyValue, Method, Request, RequestBody};

fn param_enabled(param: &KeyValue) -> bool {
    !param.disabled && !param.name.is_empty()
}

fn enabled_params(params: &KeyValList) -> Vec<(&String, &String)> {
    params
        .iter()
        .filter(|param| param_enabled(param))
        .map(|param| (&param.name, &param.value))
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

fn req_headers(mut builder: RequestBuilder, headers: &KeyValList) -> RequestBuilder {
    let iter = headers.iter().filter(|p| param_enabled(p));
    for header in iter {
        builder = builder.header(&header.name, &header.value);
    }
    builder
}

fn req_params(builder: RequestBuilder, params: &KeyValList) -> RequestBuilder {
    let params = enabled_params(params);
    builder.query(&params)
}

pub async fn transform_request(
    client: reqwest::Client,
    req: Request,
) -> anyhow::Result<reqwest::Request> {
    let url = replace_path_params(&req.url, &req.path_params);

    let mut builder = client.request(req_method(req.method), url);

    builder = req_headers(builder, &req.headers);
    builder = req_params(builder, &req.query_params);
    builder = req_body(builder, req.body).await;

    builder.build().context("Failed to build request")
}

fn replace_path_params(url: &str, params: &KeyValList) -> String {
    let mut url = url.to_string();
    for param in params {
        url = url.replace(&format!(":{}", param.name), &param.value);
    }
    url
}

async fn req_body(builder: RequestBuilder, body: RequestBody) -> RequestBuilder {
    match body {
        RequestBody::Text(text) => builder.body(text).header(CONTENT_TYPE, TEXT_PLAIN.as_ref()),
        RequestBody::Json(json) => builder
            .body(json)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref()),
        RequestBody::XML(xml) => builder.body(xml).header(CONTENT_TYPE, TEXT_XML.as_ref()),
        RequestBody::Form(form) => builder.form(&enabled_params(&form)),
        RequestBody::File(Some(file)) => {
            let content_type = mime_guess::from_path(&file)
                .first_or_octet_stream()
                .to_string();

            let file = tokio::fs::OpenOptions::new()
                .read(true)
                .open(file)
                .await
                .expect("Failed to open file");
            dbg!(&file);
            builder.body(file).header(CONTENT_TYPE, content_type)
        }
        RequestBody::None | RequestBody::File(None) => builder,
    }
}
