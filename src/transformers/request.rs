use anyhow::Context;
use mime::{APPLICATION_JSON, APPLICATION_WWW_FORM_URLENCODED, TEXT_PLAIN, TEXT_XML};
use mime_guess::mime;
use reqwest::header::CONTENT_TYPE;
use reqwest::RequestBuilder;

use crate::components::KeyValue;
use crate::state::request::{Request, RequestBody};
use crate::{components::KeyValList, state::request};

fn param_enabled(param: &KeyValue) -> bool {
    !param.disabled && !param.name.is_empty()
}

fn enabled_params(params: &KeyValList) -> Vec<(&String, &String)> {
    params
        .values()
        .iter()
        .filter(|param| param_enabled(param))
        .map(|param| (&param.name, &param.value))
        .collect()
}

fn req_method(method: request::Method) -> reqwest::Method {
    match method {
        request::Method::GET => reqwest::Method::GET,
        request::Method::POST => reqwest::Method::POST,
        request::Method::PUT => reqwest::Method::PUT,
        request::Method::DELETE => reqwest::Method::DELETE,
    }
}

fn req_headers(mut builder: RequestBuilder, headers: &KeyValList) -> RequestBuilder {
    let iter = headers.values().iter().filter(|p| param_enabled(p));
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
    let mut builder = client.request(req_method(req.method), &req.url);

    builder = req_headers(builder, &req.headers);
    builder = req_params(builder, &req.query_params);
    builder = req_body(builder, req.body).await;

    builder.build().context("Failed to build request")
}

async fn req_body(builder: RequestBuilder, body: RequestBody) -> RequestBuilder {
    match body {
        RequestBody::None => builder,
        RequestBody::Text(text) => builder.body(text).header(CONTENT_TYPE, TEXT_PLAIN.as_ref()),
        RequestBody::Json(json) => builder
            .body(json)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref()),
        RequestBody::XML(xml) => builder.body(xml).header(CONTENT_TYPE, TEXT_XML.as_ref()),
        RequestBody::Form(form) => {
            let form = enabled_params(&form);
            builder
                .form(&form)
                .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
        }
        RequestBody::File(file) => {
            let content_type = mime_guess::from_path(&file)
                .first_or_octet_stream()
                .to_string();

            let file = tokio::fs::OpenOptions::new()
                .read(true)
                .open(file)
                .await
                .expect("Failed to open file");
            builder.body(file).header(CONTENT_TYPE, content_type)
        }
    }
}
