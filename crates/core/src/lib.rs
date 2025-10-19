#![allow(mismatched_lifetime_syntaxes)]
pub mod assertions;
pub mod client;
pub mod curl;
pub mod http;
pub mod ids;
pub mod import;
pub mod perf;
pub mod persistence;
pub mod scripting;
pub mod transformers;
pub mod utils;

pub const APP_NAME: &str = "Sanchaar";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
