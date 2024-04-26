use serde::{Deserialize, Serialize};

pub mod collections;
pub mod request;
mod environment;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Version {
    V1,
}
