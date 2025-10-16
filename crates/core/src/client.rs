use std::{fmt::Display, str::FromStr, sync::Arc, time::Duration};

use reqwest::{Client, Request, StatusCode, header::HeaderMap};
use reqwest_cookie_store::CookieStoreRwLock;

fn is_json_content_type(content_type: &str) -> bool {
    let content_type = content_type.to_lowercase();
    let main_type = content_type.split(';').next().unwrap_or("").trim();
    matches!(main_type, "application/json" | "text/json") || main_type.ends_with("+json")
}

fn is_html_content_type(content_type: &str) -> bool {
    let content_type = content_type.to_lowercase();
    let main_type = content_type.split(';').next().unwrap_or("").trim();
    matches!(main_type, "text/html") || main_type.ends_with("+html")
}

fn is_xml_content_type(content_type: &str) -> bool {
    let content_type = content_type.to_lowercase();
    let main_type = content_type.split(';').next().unwrap_or("").trim();
    matches!(main_type, "application/xml" | "text/xml") || main_type.ends_with("+xml")
}

fn is_text_content_type(content_type: &str) -> bool {
    let content_type = content_type.to_lowercase();
    let main_type = content_type.split(';').next().unwrap_or("").trim();

    matches!(
        main_type,
        "text/plain" | "text/css" | "text/javascript" | "text/csv" | "text/yaml" | "text/markdown"
    ) || main_type.starts_with("text/")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Json,
    Text,
    XML,
    Html,
    Buffer,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Json => "json",
            ContentType::Text => "text",
            ContentType::Html => "html",
            ContentType::XML => "xml",
            ContentType::Buffer => "buffer",
        }
    }
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for ContentType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ContentType::Json),
            "text" => Ok(ContentType::Text),
            "html" => Ok(ContentType::Html),
            "xml" => Ok(ContentType::XML),
            "buffer" => Ok(ContentType::Buffer),
            _ => Err(anyhow::anyhow!("Invalid content type: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseBody {
    pub content_type: ContentType,
    pub data: Arc<Vec<u8>>,
}

impl ResponseBody {
    pub fn is_json(&self) -> bool {
        self.content_type == ContentType::Json
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: ResponseBody,
    pub duration: Duration,
    pub size_bytes: usize,
}

pub async fn send_request(client: Client, req: Request) -> anyhow::Result<Response> {
    let start = std::time::Instant::now();
    let res = client.execute(req).await?;
    let duration = start.elapsed();
    let status = res.status();
    let headers = res.headers().clone();

    let content_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let data = res.bytes().await?.to_vec().into();

    let content_type = if is_json_content_type(content_type) {
        ContentType::Json
    } else if is_text_content_type(content_type) {
        ContentType::Text
    } else if is_html_content_type(content_type) {
        ContentType::Html
    } else if is_xml_content_type(content_type) {
        ContentType::XML
    } else {
        ContentType::Buffer
    };

    let body = ResponseBody { content_type, data };

    let size_bytes = body.data.len();

    Ok(Response {
        status,
        headers,
        body,
        duration,
        size_bytes,
    })
}

pub fn create_cookie_store() -> Arc<CookieStoreRwLock> {
    Arc::new(CookieStoreRwLock::default())
}

pub fn create_client(disable_verification: bool, store: Arc<CookieStoreRwLock>) -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(disable_verification)
        .cookie_provider(store)
        .build()
        .expect("Failed to create client")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_content_type_detection() {
        // Standard JSON types
        assert!(is_json_content_type("application/json"));
        assert!(is_json_content_type("text/json"));

        // JSON with charset
        assert!(is_json_content_type("application/json; charset=utf-8"));
        assert!(is_json_content_type("application/json;charset=utf-8"));

        // Specific JSON variants
        assert!(is_json_content_type("application/ld+json"));
        assert!(is_json_content_type("application/hal+json"));
        assert!(is_json_content_type("application/vnd.api+json"));
        assert!(is_json_content_type("application/problem+json"));
        assert!(is_json_content_type("application/merge-patch+json"));
        assert!(is_json_content_type("application/json-patch+json"));

        // Generic +json types
        assert!(is_json_content_type("application/vnd.github.v3+json"));
        assert!(is_json_content_type("application/custom+json"));

        // Case insensitive
        assert!(is_json_content_type("APPLICATION/JSON"));
        assert!(is_json_content_type("Application/Json"));

        // Not JSON
        assert!(!is_json_content_type("text/plain"));
        assert!(!is_json_content_type("text/html"));
        assert!(!is_json_content_type("application/xml"));
        assert!(!is_json_content_type("application/octet-stream"));
    }

    #[test]
    fn test_text_content_type_detection() {
        // Standard text types
        assert!(is_text_content_type("text/plain"));
        assert!(is_text_content_type("text/html"));
        assert!(is_text_content_type("text/css"));
        assert!(is_text_content_type("text/javascript"));
        assert!(is_text_content_type("text/csv"));
        assert!(is_text_content_type("text/xml"));
        assert!(is_text_content_type("text/yaml"));
        assert!(is_text_content_type("text/markdown"));

        // Text with charset
        assert!(is_text_content_type("text/plain; charset=utf-8"));
        assert!(is_text_content_type("text/html;charset=iso-8859-1"));

        // Generic text/ types
        assert!(is_text_content_type("text/custom"));
        assert!(is_text_content_type("text/whatever"));

        // Case insensitive
        assert!(is_text_content_type("TEXT/PLAIN"));
        assert!(is_text_content_type("Text/Html"));

        // Not text
        assert!(!is_text_content_type("application/json"));
        assert!(!is_text_content_type("application/xml"));
        assert!(!is_text_content_type("image/png"));
        assert!(!is_text_content_type("application/octet-stream"));
    }
}
