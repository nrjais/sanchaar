use serde::{Deserialize, Serialize};
use std::{ops::Not, path::PathBuf};

use crate::http::KeyValue;

pub mod collections;
pub mod environment;
pub mod request;

pub const TOML_EXTENSION: &str = ".toml";
pub const JS_EXTENSION: &str = "js";
pub const TS_EXTENSION: &str = "ts";
pub const COLLECTION_ROOT_FILE: &str = "collection.toml";
pub const ENVIRONMENTS: &str = "environments";
pub const SCRIPTS: &str = "scripts";
pub const REQUESTS: &str = "requests";

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedKeyValue {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Not::not")]
    pub disabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedKeyFile {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Not::not")]
    pub disabled: bool,
}

impl From<KeyValue> for EncodedKeyValue {
    fn from(value: KeyValue) -> Self {
        EncodedKeyValue {
            name: value.name,
            value: value.value,
            disabled: value.disabled,
        }
    }
}

impl From<EncodedKeyValue> for KeyValue {
    fn from(value: EncodedKeyValue) -> Self {
        KeyValue {
            name: value.name,
            value: value.value,
            disabled: value.disabled,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Version {
    V1,
}
