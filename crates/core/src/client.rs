use std::{str::FromStr, sync::Arc, time::Duration};

use reqwest::{Client, Request, StatusCode, header::HeaderMap};
use reqwest_cookie_store::CookieStoreRwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Json,
    Text,
    Buffer,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Json => "json",
            ContentType::Text => "text",
            ContentType::Buffer => "buffer",
        }
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl FromStr for ContentType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(ContentType::Json),
            "text" => Ok(ContentType::Text),
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

    let body: ResponseBody = match content_type {
        "application/json" => ResponseBody {
            content_type: ContentType::Json,
            data: res.bytes().await?.to_vec().into(),
        },
        "text/plain" => ResponseBody {
            content_type: ContentType::Text,
            data: res.bytes().await?.to_vec().into(),
        },
        _ => ResponseBody {
            content_type: ContentType::Buffer,
            data: res.bytes().await?.to_vec().into(),
        },
    };

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
