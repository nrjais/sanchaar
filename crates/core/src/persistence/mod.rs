use serde::{Deserialize, Serialize};
use std::{ops::Not, path::PathBuf};
use strum::{Display, EnumString};

use crate::http::{KeyValList, KeyValue};

pub mod collections;
pub mod environment;
pub mod history;
pub mod request;

pub const HCL_SUFFIX: &str = "hcl";
pub const HCL_EXTENSION: &str = ".hcl";
pub const JS_EXTENSION: &str = "js";
pub const TS_EXTENSION: &str = "ts";
pub const COLLECTION_ROOT_FILE: &str = "collection.hcl";
pub const ENVIRONMENTS: &str = "environments";
pub const SCRIPTS: &str = "scripts";
pub const REQUESTS: &str = "requests";
pub const HISTORY_DB: &str = "history.db";

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

#[derive(Debug, Clone, Copy, Serialize, Default, Deserialize, Display, EnumString)]
pub enum Version {
    #[default]
    V1,
}

pub fn encode_key_values(kv: KeyValList) -> Vec<EncodedKeyValue> {
    kv.into_iter().map(|v| v.into()).collect()
}

pub fn decode_key_values(kv: Vec<EncodedKeyValue>) -> KeyValList {
    let mut list = Vec::new();
    for v in kv {
        list.push(KeyValue {
            name: v.name,
            value: v.value,
            disabled: v.disabled,
        });
    }

    KeyValList::from(list)
}
