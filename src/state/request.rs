use iced::widget::text_editor;
use iced_aw::split;
use std::path::PathBuf;
use strum::{Display, EnumString, VariantArray};

use crate::components::KeyValList;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReqTabId {
    #[default]
    Params,
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
pub enum RequestBody {
    Form(KeyValList),
    Json(text_editor::Content),
    XML(text_editor::Content),
    Text(String),
    File(PathBuf),
    #[default]
    None,
}

impl RequestBody {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestBody::Form(_) => "URL Encoded",
            RequestBody::Json(_) => "Json",
            RequestBody::XML(_) => "XML",
            RequestBody::Text(_) => "Text",
            RequestBody::File(_) => "File",
            RequestBody::None => "None",
        }
    }

    pub fn all_variants() -> Vec<&'static str> {
        Vec::from(["URL Encoded", "Json", "XML", "Text", "File", "None"])
    }
}

#[derive(Debug, Default)]
pub struct RequestPane {
    pub url: String,
    pub method: Method,
    pub headers: KeyValList,
    pub body: RequestBody,
    pub query_params: KeyValList,
    pub split_axis: split::Axis,
    pub split_pos: Option<u16>,
    pub tab: ReqTabId,
}

impl RequestPane {
    pub(crate) fn new() -> RequestPane {
        RequestPane {
            url: "https://echo.nrjais.com".to_string(),
            ..Default::default()
        }
    }
}
