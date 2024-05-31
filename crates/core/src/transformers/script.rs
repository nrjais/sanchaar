use js::RequestScriptCtx;

use crate::http::{
    request::{Request, RequestBody},
    KeyValList, KeyValue,
};

pub async fn execute_sript(script: String, req: Request) -> anyhow::Result<Request> {
    let ctx = RequestScriptCtx {
        request: js::Request {
            method: req.method.to_string(),
            url: req.url.to_string(),
            path_params: key_values(req.path_params.clone()),
            headers: key_values(req.headers.clone()),
            query_params: key_values(req.query_params.clone()),
            body: match &req.body {
                RequestBody::Json(v) => match serde_json::from_str(v) {
                    Ok(json) => js::RequestBody::Json(json),
                    Err(_) => js::RequestBody::Text(v.clone()),
                },
                RequestBody::Text(v) => js::RequestBody::Text(v.clone()),
                RequestBody::XML(v) => js::RequestBody::Text(v.clone()),
                RequestBody::Form(v) => js::RequestBody::Form(key_values(v.clone())),
                _ => js::RequestBody::None,
            },
            body_raw: None,
        },
        script,
    };
    let _ = js::execute_sript(ctx).await?;

    Ok(req)
}

fn key_values(req: KeyValList) -> Vec<(String, String)> {
    req.into_iter()
        .filter(|KeyValue { disabled, .. }| !disabled)
        .map(|KeyValue { name, value, .. }| (name, value))
        .collect()
}
