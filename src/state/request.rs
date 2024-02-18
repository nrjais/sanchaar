use iced_aw::split;
use strum::{Display, EnumString, VariantArray};

use crate::components::{KeyValList};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReqTabId {
    #[default]
    Queries,
    Body,
    Headers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, VariantArray, Display, Default)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Default)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: KeyValList,
    pub query_params: KeyValList,
    pub split_axis: split::Axis,
    pub split_pos: Option<u16>,
    pub tab: ReqTabId,
}

impl Request {
    pub(crate) fn new() -> Request {
        Request {
            url: "http://echo.nrjais.com".to_string(),
            ..Default::default()
        }
    }
}
