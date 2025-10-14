// generated with `cargo typify -B crates/core/src/import/postman/schema.json`
// Schema: https://schema.postman.com/collection/json/v2.1.0/draft-07/collection.json
pub mod schema;

use anyhow::{Context, Result};
use schema::*;
use serde::Deserialize;
use std::path::Path;
use tokio::fs;

use crate::http::collection::{Entry, Folder, FolderId, RequestId, RequestRef};
use crate::http::request::{
    Auth as SanchaarAuth, Method as SanchaarMethod, Request as SanchaarRequest,
    RequestBody as SanchaarRequestBody,
};
use crate::http::{KeyValList, KeyValue};
use crate::persistence::collections::{EncodedCollection, save_collection};
use crate::persistence::request::{encode_request, save_req_to_file};
use crate::persistence::{REQUESTS, TOML_EXTENSION};

/// Root Postman Collection structure
#[derive(Debug, Clone, Deserialize)]
pub struct PostmanCollection {
    pub info: Info,
    #[serde(default)]
    pub item: Vec<Items>,
    #[serde(default)]
    pub auth: Option<Auth>,
    #[serde(default)]
    pub variable: Option<VariableList>,
}

pub async fn import_postman_collection(postman_path: &Path, output_dir: &Path) -> Result<()> {
    let content = fs::read_to_string(postman_path)
        .await
        .context("Failed to read Postman collection file")?;

    let postman_collection: PostmanCollection =
        serde_json::from_str(&content).context("Failed to parse Postman collection JSON")?;

    let collection_name = sanitize_name(&postman_collection.info.name);
    let collection_dir = output_dir.join(&collection_name);
    fs::create_dir_all(&collection_dir).await?;

    let requests_dir = collection_dir.join(REQUESTS);
    fs::create_dir_all(&requests_dir).await?;

    let mut entries = Vec::new();
    for item in postman_collection.item {
        if let Some(entry) = process_item(item, &requests_dir).await? {
            entries.push(entry);
        }
    }

    // TODO: Implement collection variable extraction
    let encoded_collection = EncodedCollection {
        name: collection_name.clone(),
        version: crate::persistence::Version::V1,
        disable_cert_verification: false,
        timeout: std::time::Duration::from_secs(300),
        default_environment: None,
        headers: vec![],
    };

    save_collection(collection_dir.clone(), encoded_collection).await?;

    log::info!(
        "Successfully imported Postman collection '{}' to {:?}",
        collection_name,
        collection_dir
    );

    Ok(())
}

fn process_item(
    item: Items,
    base_path: &Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<Entry>>> + Send + '_>> {
    Box::pin(async move {
        match item {
            Items::Variant0(item) => process_request_item(item, base_path).await.map(Some),
            Items::Variant1(item_group) => process_folder(item_group, base_path).await.map(Some),
        }
    })
}

async fn process_request_item(item: Item, base_path: &Path) -> Result<Entry> {
    let name = item.name.unwrap_or_else(|| "Unnamed Request".to_string());
    let sanitized_name = sanitize_name(&name);

    let sanchaar_request = convert_request(item.request, item.description.as_ref())?;

    let request_path = base_path.join(format!("{}{}", sanitized_name, TOML_EXTENSION));
    let encoded_request = encode_request(sanchaar_request);
    save_req_to_file(request_path.clone(), encoded_request).await?;

    Ok(Entry::Item(RequestRef {
        id: RequestId::new(),
        name: sanitized_name,
        path: request_path,
    }))
}

async fn process_folder(folder: ItemGroup, base_path: &Path) -> Result<Entry> {
    let name = folder.name.unwrap_or_else(|| "Unnamed Folder".to_string());
    let sanitized_name = sanitize_name(&name);

    let folder_path = base_path.join(&sanitized_name);
    fs::create_dir_all(&folder_path).await?;

    let mut entries = Vec::new();
    for item in folder.item {
        if let Some(entry) = process_item(item, &folder_path).await? {
            entries.push(entry);
        }
    }

    Ok(Entry::Folder(Folder {
        id: FolderId::new(),
        name: sanitized_name,
        entries,
        path: folder_path,
        expanded: false,
    }))
}

fn convert_request(request: Request, description: Option<&Description>) -> Result<SanchaarRequest> {
    match request {
        Request::Request {
            url,
            method,
            header,
            body,
            auth,
            ..
        } => {
            let url_str = extract_url(url);
            let method = extract_method(method);
            let headers = extract_headers(header);
            let body = extract_body(body);
            let auth = extract_auth(auth);
            let desc = extract_description(description);
            let (query_params, path_params) = extract_params(&url_str);

            Ok(SanchaarRequest {
                description: desc,
                method,
                url: url_str,
                headers,
                body,
                query_params,
                path_params,
                auth,
                assertions: Default::default(),
                pre_request: None,
            })
        }
        Request::String(url) => Ok(SanchaarRequest {
            description: "Imported from Postman".to_string(),
            method: SanchaarMethod::GET,
            url,
            headers: KeyValList::new(),
            body: SanchaarRequestBody::None,
            query_params: KeyValList::new(),
            path_params: KeyValList::new(),
            auth: SanchaarAuth::None,
            assertions: Default::default(),
            pre_request: None,
        }),
    }
}

fn extract_url(url: Option<Url>) -> String {
    match url {
        Some(Url::String(s)) => s,
        Some(Url::Object { raw, .. }) => raw.unwrap_or_else(|| "https://example.com".to_string()),
        None => "https://example.com".to_string(),
    }
}

fn extract_method(method: Option<RequestMethod>) -> SanchaarMethod {
    method
        .and_then(|m| m.subtype_0)
        .map(|m| match m {
            RequestMethodSubtype0::Get => SanchaarMethod::GET,
            RequestMethodSubtype0::Post => SanchaarMethod::POST,
            RequestMethodSubtype0::Put => SanchaarMethod::PUT,
            RequestMethodSubtype0::Delete => SanchaarMethod::DELETE,
            RequestMethodSubtype0::Patch => SanchaarMethod::PATCH,
            RequestMethodSubtype0::Head => SanchaarMethod::HEAD,
            RequestMethodSubtype0::Options => SanchaarMethod::OPTIONS,
            _ => SanchaarMethod::GET,
        })
        .unwrap_or(SanchaarMethod::GET)
}

fn extract_headers(header: Option<RequestHeader>) -> KeyValList {
    match header {
        Some(RequestHeader::HeaderList(list)) => {
            let headers: Vec<KeyValue> = list
                .iter()
                .map(|h| KeyValue {
                    disabled: h.disabled,
                    name: h.key.clone(),
                    value: h.value.clone(),
                })
                .collect();
            KeyValList::from(headers)
        }
        _ => KeyValList::new(),
    }
}

fn extract_body(body: Option<RequestBody>) -> SanchaarRequestBody {
    match body {
        Some(body) => match body.mode {
            Some(RequestBodyMode::Raw) => SanchaarRequestBody::Json(body.raw.unwrap_or_default()),
            Some(RequestBodyMode::Urlencoded) => {
                let params: Vec<KeyValue> = body
                    .urlencoded
                    .iter()
                    .map(|p| KeyValue {
                        disabled: p.disabled,
                        name: p.key.clone(),
                        value: p.value.clone().unwrap_or_default(),
                    })
                    .collect();
                SanchaarRequestBody::Form(KeyValList::from(params))
            }
            Some(RequestBodyMode::Formdata) => {
                let params: Vec<KeyValue> = body
                    .formdata
                    .iter()
                    .filter_map(|p| {
                        if let Some(subtype0) = &p.subtype_0 {
                            Some(KeyValue {
                                disabled: subtype0.disabled,
                                name: subtype0.key.clone(),
                                value: subtype0.value.clone().unwrap_or_default(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                SanchaarRequestBody::Form(KeyValList::from(params))
            }
            Some(RequestBodyMode::File) => SanchaarRequestBody::File(None),
            _ => SanchaarRequestBody::None,
        },
        None => SanchaarRequestBody::None,
    }
}

fn extract_auth(auth: Option<Auth>) -> SanchaarAuth {
    match auth {
        Some(auth) => match auth.type_ {
            AuthType::Basic => {
                let username = auth
                    .basic
                    .iter()
                    .find(|a| a.key == "username")
                    .and_then(|a| a.value.as_ref())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let password = auth
                    .basic
                    .iter()
                    .find(|a| a.key == "password")
                    .and_then(|a| a.value.as_ref())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                SanchaarAuth::Basic { username, password }
            }
            AuthType::Bearer => {
                let token = auth
                    .bearer
                    .iter()
                    .find(|a| a.key == "token")
                    .and_then(|a| a.value.as_ref())
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                SanchaarAuth::Bearer { token }
            }
            _ => SanchaarAuth::None,
        },
        None => SanchaarAuth::None,
    }
}

fn extract_description(desc: Option<&Description>) -> String {
    match desc {
        Some(Description::String(s)) => s.clone(),
        Some(Description::Description { content, .. }) => content.clone().unwrap_or_default(),
        _ => "Imported from Postman".to_string(),
    }
}

fn extract_params(url_str: &str) -> (KeyValList, KeyValList) {
    let mut query_params = Vec::new();
    let path_params = Vec::new();

    if let Some(query_start) = url_str.find('?') {
        let query_string = &url_str[query_start + 1..];
        for param in query_string.split('&') {
            if let Some(eq_pos) = param.find('=') {
                let name = param[..eq_pos].to_string();
                let value = param[eq_pos + 1..].to_string();
                query_params.push(KeyValue {
                    disabled: false,
                    name,
                    value,
                });
            } else if !param.is_empty() {
                query_params.push(KeyValue {
                    disabled: false,
                    name: param.to_string(),
                    value: String::new(),
                });
            }
        }
    }

    (
        KeyValList::from(query_params),
        KeyValList::from(path_params),
    )
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}
