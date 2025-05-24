use std::time::Duration;

use crate::http::collection::Collection;
use crate::persistence::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds};
use std::ops::Not;

use super::database::DatabaseManager;
use super::{encode_key_values, EncodedKeyValue};

fn default_timeout() -> Duration {
    Duration::from_secs(300)
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct EncodedCollection {
    pub name: String,
    pub version: Version,
    #[serde(default, skip_serializing_if = "Not::not")]
    pub disable_cert_verification: bool,
    #[serde(default = "default_timeout")]
    #[serde_as(as = "DurationMilliSeconds")]
    pub timeout: Duration,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_environment: Option<String>,
    #[serde(default)]
    pub headers: Vec<EncodedKeyValue>,
    #[serde(default)]
    pub variables: Vec<EncodedKeyValue>,
}

pub fn load_collections_from_database() -> Result<Vec<Collection>> {
    let mut db_manager = DatabaseManager::new()?;
    db_manager.scan_and_load_collections()
}

pub fn create_collection_in_database(name: String) -> Result<String> {
    let mut db_manager = DatabaseManager::new()?;
    db_manager.create_collection(name)
}

pub fn encode_collection(collection: &Collection) -> EncodedCollection {
    EncodedCollection {
        name: collection.name.clone(),
        version: Version::V1,
        disable_cert_verification: collection.disable_ssl,
        timeout: collection.timeout,
        default_environment: collection
            .default_env
            .as_ref()
            .and_then(|env| collection.environments.get(env.clone()))
            .map(|env| env.name.clone()),
        headers: encode_key_values((*collection.headers).clone()),
        variables: encode_key_values((*collection.variables).clone()),
    }
}
