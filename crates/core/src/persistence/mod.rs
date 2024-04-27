use serde::{Deserialize, Serialize};

pub mod collections;
mod environment;
pub mod request;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Version {
    V1,
}
