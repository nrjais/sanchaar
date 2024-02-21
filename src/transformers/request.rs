use anyhow::Context;
use reqwest::RequestBuilder;

use crate::{components::KeyValList, state::request};

fn req_method(method: request::Method) -> reqwest::Method {
    match method {
        request::Method::GET => reqwest::Method::GET,
        request::Method::POST => reqwest::Method::POST,
        request::Method::PUT => reqwest::Method::PUT,
        request::Method::DELETE => reqwest::Method::DELETE,
    }
}

fn req_headers(mut builder: RequestBuilder, headers: &KeyValList) -> RequestBuilder {
    for header in headers.values() {
        if !header.disabled && !header.name.is_empty() {
            builder = builder.header(&header.name, &header.value);
        }
    }
    builder
}

fn req_params(builder: RequestBuilder, params: &KeyValList) -> RequestBuilder {
    let params = params
        .values()
        .iter()
        .filter(|param| !param.disabled && !param.name.is_empty())
        .map(|param| (param.name.as_str(), param.value.as_str()))
        .collect::<Vec<_>>();
    builder.query(&params)
}

pub fn transform_request(
    client: &reqwest::Client,
    req: &request::RequestPane,
) -> anyhow::Result<reqwest::Request> {
    let mut builder = client.request(req_method(req.method), &req.url);

    builder = req_headers(builder, &req.headers);
    builder = req_params(builder, &req.query_params);

    builder.build().context("Failed to build request")
}
