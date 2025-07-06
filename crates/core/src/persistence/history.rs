use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use crate::client::{ContentType, Response, ResponseBody};
use crate::http::KeyValList;
use crate::http::request::{Auth, Method, Request, RequestBody};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub headers: String,
    pub body: String,
    pub auth: String,
    pub query_params: String,
    pub path_params: String,
    pub description: String,
    pub response_status: i32,
    pub response_headers: String,
    pub response_body: Vec<u8>,
    pub response_duration_ms: i64,
    pub response_size_bytes: i64,
    pub response_content_type: String,
    pub collection_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HistoryEntrySummary {
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub response_status: i32,
    pub response_duration_ms: i64,
    pub response_size_bytes: i64,
    pub collection_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HistoryDatabase {
    pool: SqlitePool,
}

impl HistoryDatabase {
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = format!("sqlite:{}?mode=rwc", db_path.display());
        let pool = SqlitePool::connect(&database_url).await?;

        // Create the initial table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                headers TEXT NOT NULL,
                body TEXT NOT NULL,
                auth TEXT NOT NULL,
                query_params TEXT NOT NULL,
                path_params TEXT NOT NULL,
                description TEXT NOT NULL,
                response_status INTEGER NOT NULL,
                response_headers TEXT NOT NULL,
                response_body BLOB NOT NULL,
                response_duration_ms INTEGER NOT NULL,
                response_size_bytes INTEGER NOT NULL,
                collection_name TEXT
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Add the response_content_type column if it doesn't exist (migration)
        let _ = sqlx::query(
            r#"
            ALTER TABLE history ADD COLUMN response_content_type TEXT DEFAULT 'buffer'
            "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS history_fts USING fts5(
                id UNINDEXED,
                method,
                url,
                body,
                description,
                content='history',
                content_rowid='id'
            )
            "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS history_fts_insert AFTER INSERT ON history BEGIN
                INSERT INTO history_fts(id, method, url, body, description)
                VALUES (new.id, new.method, new.url, new.body, new.description);
            END
            "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS history_fts_delete AFTER DELETE ON history BEGIN
                DELETE FROM history_fts WHERE id = old.id;
            END
            "#,
        )
        .execute(&pool)
        .await;

        Ok(Self { pool })
    }

    pub async fn save_request_response(
        &self,
        request: &Request,
        response: &Response,
        collection_name: Option<&str>,
    ) -> Result<i64> {
        let timestamp = Utc::now();
        let headers = serde_json::to_string(&request.headers)?;
        let body = serde_json::to_string(&request.body)?;
        let auth = serde_json::to_string(&request.auth)?;
        let query_params = serde_json::to_string(&request.query_params)?;
        let path_params = serde_json::to_string(&request.path_params)?;

        // Convert HeaderMap to HashMap<String, String> for serialization
        let response_headers_map: HashMap<String, String> = response
            .headers
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let response_headers = serde_json::to_string(&response_headers_map)?;

        let result = sqlx::query(
            r#"
            INSERT INTO history (
                timestamp, method, url, headers, body, auth, query_params, path_params,
                description, response_status, response_headers, response_body,
                response_duration_ms, response_size_bytes, response_content_type, collection_name
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(timestamp.to_rfc3339())
        .bind(request.method.to_string())
        .bind(&request.url)
        .bind(headers)
        .bind(body)
        .bind(auth)
        .bind(query_params)
        .bind(path_params)
        .bind(&request.description)
        .bind(response.status.as_u16() as i32)
        .bind(response_headers)
        .bind(response.body.data.as_ref())
        .bind(response.duration.as_millis() as i64)
        .bind(response.size_bytes as i64)
        .bind(response.body.content_type.to_string())
        .bind(collection_name)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn get_history(&self, limit: Option<i64>) -> Result<Vec<HistoryEntry>> {
        let limit = limit.unwrap_or(100);

        let rows = sqlx::query(
            r#"
            SELECT
                id,
                timestamp,
                method,
                url,
                headers,
                body,
                auth,
                query_params,
                path_params,
                description,
                response_status,
                response_headers,
                response_body,
                response_duration_ms,
                response_size_bytes,
                response_content_type,
                collection_name
            FROM history
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut entries = Vec::new();
        for row in rows {
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?.with_timezone(&Utc);

            let entry = HistoryEntry {
                id: row.try_get("id")?,
                timestamp,
                method: row.try_get("method")?,
                url: row.try_get("url")?,
                headers: row.try_get("headers")?,
                body: row.try_get("body")?,
                auth: row.try_get("auth")?,
                query_params: row.try_get("query_params")?,
                path_params: row.try_get("path_params")?,
                description: row.try_get("description")?,
                response_status: row.try_get("response_status")?,
                response_headers: row.try_get("response_headers")?,
                response_body: row.try_get("response_body")?,
                response_duration_ms: row.try_get("response_duration_ms")?,
                response_size_bytes: row.try_get("response_size_bytes")?,
                response_content_type: row
                    .try_get("response_content_type")
                    .unwrap_or_else(|_| "buffer".to_string()),
                collection_name: row.try_get("collection_name")?,
            };
            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn get_history_summary(
        &self,
        limit: Option<i64>,
    ) -> Result<Vec<HistoryEntrySummary>> {
        let limit = limit.unwrap_or(100);

        let rows = sqlx::query(
            r#"
            SELECT
                id,
                timestamp,
                method,
                url,
                response_status,
                response_duration_ms,
                response_size_bytes,
                collection_name
            FROM history
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut entries = Vec::new();
        for row in rows {
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?.with_timezone(&Utc);

            let entry = HistoryEntrySummary {
                id: row.try_get("id")?,
                timestamp,
                method: row.try_get("method")?,
                url: row.try_get("url")?,
                response_status: row.try_get("response_status")?,
                response_duration_ms: row.try_get("response_duration_ms")?,
                response_size_bytes: row.try_get("response_size_bytes")?,
                collection_name: row.try_get("collection_name")?,
            };
            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn search_history_summary(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> Result<Vec<HistoryEntrySummary>> {
        let limit = limit.unwrap_or(100);

        if query.trim().is_empty() {
            // If no search query, return regular summary
            return self.get_history_summary(Some(limit)).await;
        }

        // Use FTS for full-text search
        let rows = sqlx::query(
            r#"
            SELECT
                h.id,
                h.timestamp,
                h.method,
                h.url,
                h.response_status,
                h.response_duration_ms,
                h.response_size_bytes,
                h.collection_name
            FROM history h
            INNER JOIN history_fts fts ON h.id = fts.id
            WHERE history_fts MATCH ?
            ORDER BY h.timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(query)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut entries = Vec::new();
        for row in rows {
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?.with_timezone(&Utc);

            let entry = HistoryEntrySummary {
                id: row.try_get("id")?,
                timestamp,
                method: row.try_get("method")?,
                url: row.try_get("url")?,
                response_status: row.try_get("response_status")?,
                response_duration_ms: row.try_get("response_duration_ms")?,
                response_size_bytes: row.try_get("response_size_bytes")?,
                collection_name: row.try_get("collection_name")?,
            };
            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn get_history_by_id(&self, id: i64) -> Result<Option<HistoryEntry>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                timestamp,
                method,
                url,
                headers,
                body,
                auth,
                query_params,
                path_params,
                description,
                response_status,
                response_headers,
                response_body,
                response_duration_ms,
                response_size_bytes,
                response_content_type,
                collection_name
            FROM history
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)?.with_timezone(&Utc);

            let entry = HistoryEntry {
                id: row.try_get("id")?,
                timestamp,
                method: row.try_get("method")?,
                url: row.try_get("url")?,
                headers: row.try_get("headers")?,
                body: row.try_get("body")?,
                auth: row.try_get("auth")?,
                query_params: row.try_get("query_params")?,
                path_params: row.try_get("path_params")?,
                description: row.try_get("description")?,
                response_status: row.try_get("response_status")?,
                response_headers: row.try_get("response_headers")?,
                response_body: row.try_get("response_body")?,
                response_duration_ms: row.try_get("response_duration_ms")?,
                response_size_bytes: row.try_get("response_size_bytes")?,
                response_content_type: row
                    .try_get("response_content_type")
                    .unwrap_or_else(|_| "buffer".to_string()),
                collection_name: row.try_get("collection_name")?,
            };
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_history_entry(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM history WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn clear_history(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM history")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }
}

impl HistoryEntry {
    pub fn to_request(&self) -> Result<Request> {
        let headers: KeyValList = serde_json::from_str(&self.headers)?;
        let body: RequestBody = serde_json::from_str(&self.body)?;
        let auth: Auth = serde_json::from_str(&self.auth)?;
        let query_params: KeyValList = serde_json::from_str(&self.query_params)?;
        let path_params: KeyValList = serde_json::from_str(&self.path_params)?;
        let method: Method = self
            .method
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid method"))?;

        Ok(Request {
            description: self.description.clone(),
            method,
            url: self.url.clone(),
            headers,
            body,
            auth,
            query_params,
            path_params,
            assertions: Default::default(),
            pre_request: None,
        })
    }

    pub fn to_response(&self) -> Result<Response> {
        let response_headers_map: HashMap<String, String> =
            serde_json::from_str(&self.response_headers)?;

        let mut headers = HeaderMap::new();
        for (key, value) in response_headers_map {
            if let (Ok(header_name), Ok(header_value)) =
                (key.parse::<HeaderName>(), value.parse::<HeaderValue>())
            {
                headers.insert(header_name, header_value);
            }
        }

        let status = StatusCode::from_u16(self.response_status as u16)
            .map_err(|_| anyhow::anyhow!("Invalid status code"))?;

        let duration = Duration::from_millis(self.response_duration_ms as u64);

        Ok(Response {
            status,
            headers,
            body: ResponseBody {
                content_type: ContentType::from_str(&self.response_content_type)?,
                data: Arc::new(self.response_body.clone()),
            },
            duration,
            size_bytes: self.response_size_bytes as usize,
        })
    }
}

pub fn get_history_db_path() -> Result<PathBuf> {
    let data_dir = directories::ProjectDirs::from("com", "nrjais", "sanchaar")
        .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
        .data_dir()
        .to_path_buf();

    Ok(data_dir.join(super::HISTORY_DB))
}
