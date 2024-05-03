use serde::{Deserialize, Serialize};
use std::ops::Not;

use crate::http::KeyValue;

pub mod collections;
pub mod environment;
pub mod request;

const TOML_EXTENSION: &str = ".toml";
const COLLECTION_ROOT_FILE: &str = "collection.toml";
const ENVIRONMENTS: &str = "environments";
const REQUESTS: &str = "requests";

#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedKeyValue {
    pub name: String,
    pub value: String,
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
