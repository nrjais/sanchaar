use serde::{Deserialize, Serialize};

pub mod collections;
pub mod fs;
pub mod request;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Version {
    V1,
}
