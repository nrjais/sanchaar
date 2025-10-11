use anyhow::{Context, Result};
use postman_collection::v2_1_0::{
    Auth, AuthAttribute, AuthType, Body, DescriptionUnion, Event, HeaderUnion, Host, Items, Mode,
    PathElement, RequestUnion, Spec, Url, UrlPath,
};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::http::environment::Environment;
use crate::http::request::{Auth as SanchaarAuth, Method, Request, RequestBody};
use crate::http::{KeyValList, KeyValue};
use crate::persistence::collections::EncodedCollection;
use crate::persistence::request::{encode_request, save_req_to_file};
use crate::persistence::{REQUESTS, Version};

/// Import a Postman collection from a file path
pub async fn import_postman_collection(postman_path: &Path, output_dir: &Path) -> Result<()> {
    // Read and parse the Postman collection file
    let content =
        fs::read_to_string(postman_path).context("Failed to read Postman collection file")?;
    let spec: Spec =
        serde_json::from_str(&content).context("Failed to parse Postman collection JSON")?;

    // Extract collection name
    let collection_name = spec.info.name.clone();

    // Create the output directory
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    // Convert and save the collection metadata
    let encoded_collection = create_sanchaar_collection(&collection_name);
    save_collection_file(output_dir, &encoded_collection).await?;

    // Convert and save environments
    let environments = extract_environments(&spec);
    save_environments(output_dir, environments).await?;

    // Convert and save requests
    let requests_dir = output_dir.join(REQUESTS);
    fs::create_dir_all(&requests_dir)?;

    if !spec.item.is_empty() {
        process_items(&spec.item, &requests_dir).await?;
    }

    Ok(())
}

fn create_sanchaar_collection(name: &str) -> EncodedCollection {
    EncodedCollection {
        name: name.to_string(),
        version: Version::V1,
        disable_cert_verification: false,
        timeout: std::time::Duration::from_secs(300),
        default_environment: None,
        headers: vec![],
    }
}

async fn save_collection_file(dir: &Path, collection: &EncodedCollection) -> Result<()> {
    let path = dir.join("collection.toml");
    let content = toml::to_string_pretty(collection)?;
    tokio::fs::write(path, content).await?;
    Ok(())
}

fn extract_environments(collection: &Spec) -> Vec<Environment> {
    let mut environments = Vec::new();

    // Extract variables as a default environment if they exist
    if let Some(ref variables) = collection.variable {
        if !variables.is_empty() {
            let mut vars = HashMap::new();
            for var in variables {
                if let (Some(key), Some(value)) = (&var.key, &var.value) {
                    vars.insert(key.clone(), value.to_string());
                }
            }

            if !vars.is_empty() {
                let env = Environment {
                    name: "Default".to_string(),
                    variables: std::sync::Arc::new(vars),
                };
                environments.push(env);
            }
        }
    }

    environments
}

async fn save_environments(dir: &Path, environments: Vec<Environment>) -> Result<()> {
    if environments.is_empty() {
        return Ok(());
    }

    let env_dir = dir.join("environments");
    tokio::fs::create_dir_all(&env_dir).await?;

    for env in environments {
        let env_path = env_dir.join(format!("{}.toml", env.name));
        let vars: HashMap<String, String> = env.variables.as_ref().clone();
        let content = toml::to_string_pretty(&vars)?;
        tokio::fs::write(env_path, content).await?;
    }

    Ok(())
}

async fn process_items(items: &[Items], base_dir: &Path) -> Result<()> {
    for item in items {
        if let Some(ref name) = item.name {
            // Check if this is a folder (has sub-items) or a request
            if let Some(ref sub_items) = item.item {
                // This is a folder
                let folder_path = base_dir.join(sanitize_filename(name));
                fs::create_dir_all(&folder_path)?;
                Box::pin(process_items(sub_items.as_slice(), &folder_path)).await?;
            } else if item.request.is_some() {
                // This is a request item
                process_request_item(item, base_dir, name).await?;
            }
        }
    }
    Ok(())
}

async fn process_request_item(item: &Items, base_dir: &Path, name: &str) -> Result<()> {
    // Convert the item to a request
    let request = convert_item_to_request(item)?;

    // Save the request
    let file_path = base_dir.join(format!("{}.toml", sanitize_filename(name)));
    let encoded = encode_request(request);
    save_req_to_file(file_path, encoded).await?;

    Ok(())
}

fn convert_item_to_request(item: &Items) -> Result<Request> {
    let request_union = item
        .request
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Item has no request"))?;

    // Extract method, URL, headers, etc.
    let (method, url, headers, query_params, body, auth) = match request_union {
        RequestUnion::String(url_str) => {
            // Simple GET request with just a URL
            (
                Method::GET,
                url_str.clone(),
                KeyValList::new(),
                KeyValList::new(),
                RequestBody::None,
                SanchaarAuth::None,
            )
        }
        RequestUnion::RequestClass(req) => {
            let method = convert_method(&req.method);
            let url = extract_url(&req.url);
            let headers = extract_headers(&req.header);
            let query_params = extract_query_params(&req.url);
            let body = extract_body(&req.body);
            let auth = extract_auth(&req.auth);
            (method, url, headers, query_params, body, auth)
        }
    };

    // Extract description
    let description = extract_description(item.description.as_ref());

    // Extract pre-request script
    let pre_request = extract_pre_request_script(&item.event);

    Ok(Request {
        description,
        method,
        url,
        headers,
        body,
        query_params,
        path_params: KeyValList::new(), // Postman doesn't have explicit path params
        auth,
        assertions: Default::default(),
        pre_request,
    })
}

fn convert_method(method: &Option<String>) -> Method {
    let method_str = method.as_deref().unwrap_or("GET");
    match method_str.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "HEAD" => Method::HEAD,
        "OPTIONS" => Method::OPTIONS,
        "CONNECT" => Method::CONNECT,
        "TRACE" => Method::TRACE,
        _ => Method::GET, // Default to GET for unknown methods
    }
}

fn extract_url(url: &Option<Url>) -> String {
    match url {
        Some(Url::String(s)) => s.clone(),
        Some(Url::UrlClass(url_class)) => build_url_from_class(url_class),
        None => String::new(),
    }
}

fn build_url_from_class(url_class: &postman_collection::v2_1_0::UrlClass) -> String {
    let protocol = url_class.protocol.as_deref().unwrap_or("https");

    let host = match &url_class.host {
        Some(Host::String(s)) => s.clone(),
        Some(Host::StringArray(arr)) => arr.join("."),
        None => String::new(),
    };

    let port = url_class
        .port
        .as_ref()
        .map(|p| format!(":{}", p))
        .unwrap_or_default();

    let path = match &url_class.path {
        Some(UrlPath::String(s)) => format!("/{}", s),
        Some(UrlPath::UnionArray(arr)) => {
            let path_parts: Vec<String> = arr
                .iter()
                .filter_map(|elem| match elem {
                    PathElement::String(s) => Some(s.clone()),
                    PathElement::PathClass(p) => p.value.clone(),
                })
                .collect();
            if path_parts.is_empty() {
                String::new()
            } else {
                format!("/{}", path_parts.join("/"))
            }
        }
        None => String::new(),
    };

    format!("{}://{}{}{}", protocol, host, port, path)
}

fn extract_headers(headers: &Option<HeaderUnion>) -> KeyValList {
    match headers {
        Some(HeaderUnion::HeaderArray(headers)) => {
            let mut key_vals = Vec::new();
            for header in headers {
                key_vals.push(KeyValue {
                    name: header.key.clone(),
                    value: header.value.clone(),
                    disabled: header.disabled.unwrap_or(false),
                });
            }
            KeyValList::from(key_vals)
        }
        _ => KeyValList::new(),
    }
}

fn extract_query_params(url: &Option<Url>) -> KeyValList {
    let url_class = match url {
        Some(Url::UrlClass(u)) => u,
        _ => return KeyValList::new(),
    };

    match &url_class.query {
        Some(queries) => {
            let mut key_vals = Vec::new();
            for query in queries {
                if let Some(ref key) = query.key {
                    key_vals.push(KeyValue {
                        name: key.clone(),
                        value: query.value.clone().unwrap_or_default(),
                        disabled: query.disabled.unwrap_or(false),
                    });
                }
            }
            KeyValList::from(key_vals)
        }
        None => KeyValList::new(),
    }
}

fn extract_body(body: &Option<Body>) -> RequestBody {
    let body = match body {
        Some(b) => b,
        None => return RequestBody::None,
    };

    match &body.mode {
        Some(Mode::Raw) => {
            let raw_content = body.raw.as_deref().unwrap_or("");

            // Try to determine if it's JSON, XML, or plain text
            if raw_content.trim_start().starts_with('{')
                || raw_content.trim_start().starts_with('[')
            {
                RequestBody::Json(raw_content.to_string())
            } else if raw_content.trim_start().starts_with('<') {
                RequestBody::XML(raw_content.to_string())
            } else {
                RequestBody::Text(raw_content.to_string())
            }
        }
        Some(Mode::Urlencoded) => {
            if let Some(ref urlencoded) = body.urlencoded {
                let mut params = Vec::new();
                for param in urlencoded {
                    params.push(KeyValue {
                        name: param.key.clone(),
                        value: param.value.clone().unwrap_or_default(),
                        disabled: param.disabled.unwrap_or(false),
                    });
                }
                RequestBody::Form(KeyValList::from(params))
            } else {
                RequestBody::None
            }
        }
        Some(Mode::Formdata) => {
            if let Some(ref formdata) = body.formdata {
                let mut params = Vec::new();
                let mut files = Vec::new();

                for param in formdata {
                    // Check if this is a file or a regular parameter
                    // In Postman, file uploads have a "type" field set to "file"
                    if param.src.is_some() {
                        // This is a file upload
                        files.push(crate::http::KeyFile {
                            name: param.key.clone(),
                            path: None,
                            disabled: param.disabled.unwrap_or(false),
                        });
                    } else {
                        // Regular form parameter
                        params.push(KeyValue {
                            name: param.key.clone(),
                            value: param.value.clone().unwrap_or_default(),
                            disabled: param.disabled.unwrap_or(false),
                        });
                    }
                }

                RequestBody::Multipart {
                    params: KeyValList::from(params),
                    files: crate::http::KeyFileList::from(files),
                }
            } else {
                RequestBody::None
            }
        }
        Some(Mode::File) => {
            // File upload - we'll store the path if available
            RequestBody::File(None)
        }
        _ => RequestBody::None,
    }
}

fn extract_auth(auth: &Option<Auth>) -> SanchaarAuth {
    let auth = match auth {
        Some(a) => a,
        None => return SanchaarAuth::None,
    };

    match auth.auth_type {
        AuthType::Basic => {
            let username = get_auth_attribute(&auth.basic, "username");
            let password = get_auth_attribute(&auth.basic, "password");
            SanchaarAuth::Basic { username, password }
        }
        AuthType::Bearer => {
            let token = get_auth_attribute(&auth.bearer, "token");
            SanchaarAuth::Bearer { token }
        }
        _ => SanchaarAuth::None,
    }
}

fn get_auth_attribute(attrs: &Option<Vec<AuthAttribute>>, key: &str) -> String {
    attrs
        .as_ref()
        .and_then(|attrs| {
            attrs
                .iter()
                .find(|attr| attr.key == key)
                .and_then(|attr| attr.value.as_ref())
                .map(|v| {
                    // Handle different value types
                    match v {
                        JsonValue::String(s) => s.clone(),
                        JsonValue::Number(n) => n.to_string(),
                        JsonValue::Bool(b) => b.to_string(),
                        _ => String::new(),
                    }
                })
        })
        .unwrap_or_default()
}

fn extract_description(desc: Option<&DescriptionUnion>) -> String {
    match desc {
        Some(DescriptionUnion::String(s)) => s.clone(),
        Some(DescriptionUnion::Description(d)) => d.content.clone().unwrap_or_default(),
        None => String::new(),
    }
}

fn extract_pre_request_script(events: &Option<Vec<Event>>) -> Option<String> {
    events
        .as_ref()?
        .iter()
        .find(|e| e.listen == "prerequest")
        .and_then(|e| e.script.as_ref())
        .and_then(|s| match &s.exec {
            Some(postman_collection::v2_1_0::Host::String(s)) => Some(s.clone()),
            Some(postman_collection::v2_1_0::Host::StringArray(arr)) => Some(arr.join("\n")),
            None => None,
        })
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("GET /users"), "GET _users");
        assert_eq!(sanitize_filename("Test: Request"), "Test_ Request");
    }

    #[test]
    fn test_convert_method() {
        assert_eq!(convert_method(&Some("GET".to_string())), Method::GET);
        assert_eq!(convert_method(&Some("post".to_string())), Method::POST);
        assert_eq!(convert_method(&None), Method::GET);
    }
}
