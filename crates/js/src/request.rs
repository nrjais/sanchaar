use rustyscript::deno_core::{extension, op2, OpState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RequestBody {
    Json(serde_json::Value),
    Text(String),
    Form(Vec<(String, String)>),
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub path_params: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub query_params: Vec<(String, String)>,
    pub body: RequestBody,
    pub body_raw: Option<String>,
}

#[op2(fast)]
#[bigint]
fn op_add_example(#[bigint] a: i64, #[bigint] b: i64) -> i64 {
    a + b
}

#[op2]
#[serde]
fn op_get_request(state: &mut OpState) -> Request {
    let req = state.borrow::<Request>();
    req.clone()
}

extension!(
    sanchaar_extension,
    ops = [op_add_example],
    esm_entry_point = "ext:request_extension/sanchaar.js",
    esm = [ dir "src/extensions", "sanchaar.js" ],
);
