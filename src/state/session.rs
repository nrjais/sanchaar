use anyhow::{Context, Result};
use lib::http::request::{Auth, AuthIn, Method, Request, RequestBody};
use lib::http::{self, CollectionKey, CollectionRequest, KeyFileList, KeyValList, RequestId};
use lib::perf::PerfConfig;
use lib::persistence::collections::project_dirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::fs;

use super::tabs::collection_tab::CollectionTab;
use super::tabs::cookies_tab::CookiesTab;
use super::tabs::history_tab::HistoryTab;
use super::{AppState, HttpTab, PaneConfig, Tab, TabKey};
use crate::components::split::Direction;
use crate::state::tabs::perf_tab::PerfTab;

const SESSION_STATE_FILE: &str = "session_state.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableKeyValue {
    pub name: String,
    pub value: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableKeyFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub disabled: bool,
}

fn serialize_kv_list(list: &KeyValList) -> Vec<SerializableKeyValue> {
    list.iter()
        .map(|kv| SerializableKeyValue {
            name: kv.name.clone(),
            value: kv.value.clone(),
            disabled: kv.disabled,
        })
        .collect()
}

fn deserialize_kv_list(list: Vec<SerializableKeyValue>) -> KeyValList {
    KeyValList::from(
        list.into_iter()
            .map(|kv| http::KeyValue {
                name: kv.name,
                value: kv.value,
                disabled: kv.disabled,
            })
            .collect(),
    )
}

fn serialize_kf_list(list: &KeyFileList) -> Vec<SerializableKeyFile> {
    list.iter()
        .map(|kf| SerializableKeyFile {
            name: kf.name.clone(),
            path: kf.path.clone(),
            disabled: kf.disabled,
        })
        .collect()
}

fn deserialize_kf_list(list: Vec<SerializableKeyFile>) -> KeyFileList {
    KeyFileList::from(
        list.into_iter()
            .map(|kf| http::KeyFile {
                name: kf.name,
                path: kf.path,
                disabled: kf.disabled,
            })
            .collect(),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializableAuth {
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    APIKey {
        key: String,
        value: String,
        add_to: AuthIn,
    },
    JWTBearer {
        secret: String,
        payload: String,
        key: String,
        add_to: AuthIn,
    },
}

impl From<Auth> for SerializableAuth {
    fn from(auth: Auth) -> Self {
        match auth {
            Auth::None => SerializableAuth::None,
            Auth::Basic { username, password } => SerializableAuth::Basic { username, password },
            Auth::Bearer { token } => SerializableAuth::Bearer { token },
            Auth::APIKey { key, value, add_to } => SerializableAuth::APIKey { key, value, add_to },
            Auth::JWTBearer {
                secret,
                payload,
                key,
                add_to,
            } => SerializableAuth::JWTBearer {
                secret,
                payload,
                key,
                add_to,
            },
        }
    }
}

impl From<SerializableAuth> for Auth {
    fn from(auth: SerializableAuth) -> Self {
        match auth {
            SerializableAuth::None => Auth::None,
            SerializableAuth::Basic { username, password } => Auth::Basic { username, password },
            SerializableAuth::Bearer { token } => Auth::Bearer { token },
            SerializableAuth::APIKey { key, value, add_to } => Auth::APIKey { key, value, add_to },
            SerializableAuth::JWTBearer {
                secret,
                payload,
                key,
                add_to,
            } => Auth::JWTBearer {
                secret,
                payload,
                key,
                add_to,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializableRequestBody {
    None,
    Form(Vec<SerializableKeyValue>),
    Multipart {
        params: Vec<SerializableKeyValue>,
        files: Vec<SerializableKeyFile>,
    },
    Json(String),
    XML(String),
    Text(String),
    File(Option<PathBuf>),
}

impl From<RequestBody> for SerializableRequestBody {
    fn from(body: RequestBody) -> Self {
        match body {
            RequestBody::None => SerializableRequestBody::None,
            RequestBody::Form(form) => SerializableRequestBody::Form(serialize_kv_list(&form)),
            RequestBody::Multipart { params, files } => SerializableRequestBody::Multipart {
                params: serialize_kv_list(&params),
                files: serialize_kf_list(&files),
            },
            RequestBody::Json(json) => SerializableRequestBody::Json(json),
            RequestBody::XML(xml) => SerializableRequestBody::XML(xml),
            RequestBody::Text(text) => SerializableRequestBody::Text(text),
            RequestBody::File(file) => SerializableRequestBody::File(file),
        }
    }
}

impl From<SerializableRequestBody> for RequestBody {
    fn from(body: SerializableRequestBody) -> Self {
        match body {
            SerializableRequestBody::None => RequestBody::None,
            SerializableRequestBody::Form(form) => RequestBody::Form(deserialize_kv_list(form)),
            SerializableRequestBody::Multipart { params, files } => RequestBody::Multipart {
                params: deserialize_kv_list(params),
                files: deserialize_kf_list(files),
            },
            SerializableRequestBody::Json(json) => RequestBody::Json(json),
            SerializableRequestBody::XML(xml) => RequestBody::XML(xml),
            SerializableRequestBody::Text(text) => RequestBody::Text(text),
            SerializableRequestBody::File(file) => RequestBody::File(file),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<SerializableKeyValue>,
    pub body: SerializableRequestBody,
    pub auth: SerializableAuth,
    pub query_params: Vec<SerializableKeyValue>,
    pub path_params: Vec<SerializableKeyValue>,
    pub pre_request: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableHttpTab {
    pub name: String,
    pub collection_ref: Option<CollectionRef>,
    pub request: SerializableRequest,
    pub split_at: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableCollectionTab {
    pub collection_key: CollectionKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePerfTab {
    pub split_at: f32,
    pub config: PerfConfig,
    pub request: Option<(CollectionKey, RequestId)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializableTab {
    Http(Box<SerializableHttpTab>),
    Collection(SerializableCollectionTab),
    CookieStore,
    History,
    Perf(SerializablePerfTab),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePaneConfig {
    pub at: f32,
    pub side_bar_open: bool,
}

impl From<&PaneConfig> for SerializablePaneConfig {
    fn from(config: &PaneConfig) -> Self {
        Self {
            at: config.at,
            side_bar_open: config.side_bar_open,
        }
    }
}

impl From<SerializablePaneConfig> for PaneConfig {
    fn from(config: SerializablePaneConfig) -> Self {
        Self {
            at: config.at,
            side_bar_open: config.side_bar_open,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializableDirection {
    Horizontal,
    Vertical,
}

impl From<Direction> for SerializableDirection {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Horizontal => SerializableDirection::Horizontal,
            Direction::Vertical => SerializableDirection::Vertical,
        }
    }
}

impl From<SerializableDirection> for Direction {
    fn from(direction: SerializableDirection) -> Self {
        match direction {
            SerializableDirection::Horizontal => Direction::Horizontal,
            SerializableDirection::Vertical => Direction::Vertical,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub active_tab: TabKey,
    pub tabs: Vec<(TabKey, SerializableTab)>,
    pub pane_config: SerializablePaneConfig,
    pub split_direction: SerializableDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRef {
    pub collection: CollectionKey,
    pub request: PathBuf,
}

impl SessionState {
    pub fn from_app_state(state: &AppState) -> Self {
        let tabs = state
            .tabs
            .iter()
            .filter_map(|(key, tab)| {
                let serializable_tab = match tab {
                    Tab::Http(http_tab) => {
                        let request = http_tab.request().to_request();
                        let collection_ref = state
                            .common
                            .collections
                            .get(http_tab.collection_key())
                            .map(|c| c.get_relative_path(http_tab.collection_ref.1))
                            .map(|request_path| CollectionRef {
                                collection: http_tab.collection_key(),
                                request: request_path.unwrap_or_default(),
                            });

                        Some(SerializableTab::Http(
                            SerializableHttpTab {
                                name: http_tab.name.clone(),
                                collection_ref,
                                request: SerializableRequest {
                                    method: request.method.to_string(),
                                    url: request.url,
                                    headers: serialize_kv_list(&request.headers),
                                    body: request.body.into(),
                                    auth: request.auth.into(),
                                    query_params: serialize_kv_list(&request.query_params),
                                    path_params: serialize_kv_list(&request.path_params),
                                    pre_request: request.pre_request,
                                },
                                split_at: http_tab.split_at,
                            }
                            .into(),
                        ))
                    }
                    Tab::Collection(col_tab) => {
                        Some(SerializableTab::Collection(SerializableCollectionTab {
                            collection_key: col_tab.collection_key,
                        }))
                    }
                    Tab::CookieStore(_) => Some(SerializableTab::CookieStore),
                    Tab::History(_) => Some(SerializableTab::History),
                    Tab::Perf(perf_tab) => Some(SerializableTab::Perf(SerializablePerfTab {
                        split_at: perf_tab.split_at,
                        config: perf_tab.config.clone(),
                        request: perf_tab.request.map(|request| (request.0, request.1)),
                    })),
                };
                serializable_tab.map(|tab| (*key, tab))
            })
            .collect();

        Self {
            active_tab: state.active_tab,
            tabs,
            pane_config: (&state.pane_config).into(),
            split_direction: state.split_direction.into(),
        }
    }
}

fn session_file_path() -> Result<PathBuf> {
    let dirs = project_dirs().context("Failed to find data directory")?;
    Ok(dirs.data_dir().join(SESSION_STATE_FILE))
}

pub async fn save_session_state(state: &SessionState) -> Result<()> {
    let path = session_file_path()?;
    let data = serde_json::to_string_pretty(state)?;
    fs::write(path, data).await?;
    Ok(())
}

pub async fn load_session_state() -> Result<SessionState> {
    let path = session_file_path()?;
    let data = fs::read(path).await?;
    let session: SessionState = serde_json::from_slice(&data)?;
    Ok(session)
}

impl AppState {
    pub fn restore_session(&mut self, session: SessionState) {
        self.pane_config = session.pane_config.into();
        self.split_direction = session.split_direction.into();

        for (tab_key, serializable_tab) in session.tabs {
            let tab = match serializable_tab {
                SerializableTab::Http(http_tab) => {
                    let request = Request {
                        description: "Http request".to_string(),
                        method: Method::from_str(&http_tab.request.method).unwrap_or_default(),
                        url: http_tab.request.url,
                        headers: deserialize_kv_list(http_tab.request.headers),
                        body: http_tab.request.body.into(),
                        auth: http_tab.request.auth.into(),
                        query_params: deserialize_kv_list(http_tab.request.query_params),
                        path_params: deserialize_kv_list(http_tab.request.path_params),
                        assertions: Default::default(),
                        pre_request: http_tab.request.pre_request,
                    };

                    let collection_ref = if let Some(collection_ref) = http_tab.collection_ref {
                        self.common
                            .collections
                            .get(collection_ref.collection)
                            .and_then(|c| c.from_relative_path(&collection_ref.request))
                            .map(|id| CollectionRequest(collection_ref.collection, id))
                            .unwrap_or_default()
                    } else {
                        CollectionRequest::default()
                    };

                    let mut tab = HttpTab::new(&http_tab.name, request, collection_ref);
                    tab.split_at = http_tab.split_at;

                    Tab::Http(tab)
                }
                SerializableTab::Collection(col_tab) => {
                    if let Some(collection) = self.common.collections.get(col_tab.collection_key) {
                        Tab::Collection(CollectionTab::new(col_tab.collection_key, collection))
                    } else {
                        continue;
                    }
                }
                SerializableTab::CookieStore => Tab::CookieStore(CookiesTab::new(&self.common)),
                SerializableTab::History => Tab::History(HistoryTab::new()),
                SerializableTab::Perf(session) => {
                    let mut tab = PerfTab::new();
                    tab.set_split_at(session.split_at);
                    tab.config = session.config;
                    tab.request = session
                        .request
                        .map(|(col_key, req_id)| CollectionRequest(col_key, req_id));
                    Tab::Perf(Box::new(tab))
                }
            };

            self.tabs.insert(tab_key, tab);
        }

        self.switch_tab(session.active_tab);
    }
}
