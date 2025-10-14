#![allow(unused_variables)]
#![allow(clippy::large_enum_variant)]

use ::std::{
    convert::{From, TryFrom},
    default::Default,
    fmt::{self, Display, Formatter},
    ops::Deref,
    option::Option,
    result::Result,
    str::FromStr,
    string::String,
    vec::Vec,
};

use ::serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use self::error::ConversionError;

pub mod error {
    use ::std::{
        error::Error,
        fmt::{self, Debug, Display, Formatter},
    };

    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl Error for ConversionError {}
    impl Display for ConversionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            Display::fmt(&self.0, f)
        }
    }
    impl Debug for ConversionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Auth {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apikey: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub awsv4: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub basic: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bearer: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub digest: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edgegrid: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hawk: Vec<AuthAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noauth: Option<::serde_json::Value>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ntlm: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub oauth1: Vec<AuthAttribute>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2: Vec<AuthAttribute>,
    #[serde(rename = "type")]
    pub type_: AuthType,
}
impl From<&Auth> for Auth {
    fn from(value: &Auth) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthAttribute {
    pub key: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<::serde_json::Value>,
}
impl From<&AuthAttribute> for AuthAttribute {
    fn from(value: &AuthAttribute) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AuthType {
    #[serde(rename = "apikey")]
    Apikey,
    #[serde(rename = "awsv4")]
    Awsv4,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "bearer")]
    Bearer,
    #[serde(rename = "digest")]
    Digest,
    #[serde(rename = "edgegrid")]
    Edgegrid,
    #[serde(rename = "hawk")]
    Hawk,
    #[serde(rename = "noauth")]
    Noauth,
    #[serde(rename = "oauth1")]
    Oauth1,
    #[serde(rename = "oauth2")]
    Oauth2,
    #[serde(rename = "ntlm")]
    Ntlm,
}
impl From<&Self> for AuthType {
    fn from(value: &AuthType) -> Self {
        *value
    }
}
impl Display for AuthType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Apikey => f.write_str("apikey"),
            Self::Awsv4 => f.write_str("awsv4"),
            Self::Basic => f.write_str("basic"),
            Self::Bearer => f.write_str("bearer"),
            Self::Digest => f.write_str("digest"),
            Self::Edgegrid => f.write_str("edgegrid"),
            Self::Hawk => f.write_str("hawk"),
            Self::Noauth => f.write_str("noauth"),
            Self::Oauth1 => f.write_str("oauth1"),
            Self::Oauth2 => f.write_str("oauth2"),
            Self::Ntlm => f.write_str("ntlm"),
        }
    }
}
impl FromStr for AuthType {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, ConversionError> {
        match value {
            "apikey" => Ok(Self::Apikey),
            "awsv4" => Ok(Self::Awsv4),
            "basic" => Ok(Self::Basic),
            "bearer" => Ok(Self::Bearer),
            "digest" => Ok(Self::Digest),
            "edgegrid" => Ok(Self::Edgegrid),
            "hawk" => Ok(Self::Hawk),
            "noauth" => Ok(Self::Noauth),
            "oauth1" => Ok(Self::Oauth1),
            "oauth2" => Ok(Self::Oauth2),
            "ntlm" => Ok(Self::Ntlm),
            _ => Err("invalid value".into()),
        }
    }
}
impl TryFrom<&str> for AuthType {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<&String> for AuthType {
    type Error = ConversionError;
    fn try_from(value: &String) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<String> for AuthType {
    type Error = ConversionError;
    fn try_from(value: String) -> Result<Self, ConversionError> {
        value.parse()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Certificate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<CertificateCert>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<CertificateKey>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub matches: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
}
impl From<&Certificate> for Certificate {
    fn from(value: &Certificate) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CertificateCert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<::serde_json::Value>,
}
impl From<&CertificateCert> for CertificateCert {
    fn from(value: &CertificateCert) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CertificateKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<::serde_json::Value>,
}
impl From<&CertificateKey> for CertificateKey {
    fn from(value: &CertificateKey) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CertificateList(pub Vec<Certificate>);
impl Deref for CertificateList {
    type Target = Vec<Certificate>;
    fn deref(&self) -> &Vec<Certificate> {
        &self.0
    }
}
impl From<CertificateList> for Vec<Certificate> {
    fn from(value: CertificateList) -> Self {
        value.0
    }
}
impl From<&CertificateList> for CertificateList {
    fn from(value: &CertificateList) -> Self {
        value.clone()
    }
}
impl From<Vec<Certificate>> for CertificateList {
    fn from(value: Vec<Certificate>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Cookie {
    pub domain: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<Value>,

    #[serde(rename = "hostOnly", default, skip_serializing_if = "Option::is_none")]
    pub host_only: Option<bool>,

    #[serde(rename = "httpOnly", default, skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub path: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl From<&Cookie> for Cookie {
    fn from(value: &Cookie) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CookieList(pub Vec<Cookie>);
impl Deref for CookieList {
    type Target = Vec<Cookie>;
    fn deref(&self) -> &Vec<Cookie> {
        &self.0
    }
}
impl From<CookieList> for Vec<Cookie> {
    fn from(value: CookieList) -> Self {
        value.0
    }
}
impl From<&CookieList> for CookieList {
    fn from(value: &CookieList) -> Self {
        value.clone()
    }
}
impl From<Vec<Cookie>> for CookieList {
    fn from(value: Vec<Cookie>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Description {
    Description {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        content: Option<String>,

        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        version: Option<::serde_json::Value>,
    },
    String(String),
    Null,
}
impl From<&Self> for Description {
    fn from(value: &Description) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Event {
    #[serde(default)]
    pub disabled: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub listen: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<Script>,
}
impl From<&Event> for Event {
    fn from(value: &Event) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EventList(pub Vec<Event>);
impl Deref for EventList {
    type Target = Vec<Event>;
    fn deref(&self) -> &Vec<Event> {
        &self.0
    }
}
impl From<EventList> for Vec<Event> {
    fn from(value: EventList) -> Self {
        value.0
    }
}
impl From<&EventList> for EventList {
    fn from(value: &EventList) -> Self {
        value.clone()
    }
}
impl From<Vec<Event>> for EventList {
    fn from(value: Vec<Event>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct FormParameter {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<FormParameterSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<FormParameterSubtype1>,
}
impl From<&FormParameter> for FormParameter {
    fn from(value: &FormParameter) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FormParameterSubtype0 {
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    #[serde(default)]
    pub disabled: bool,
    pub key: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&FormParameterSubtype0> for FormParameterSubtype0 {
    fn from(value: &FormParameterSubtype0) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FormParameterSubtype1 {
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    #[serde(default)]
    pub disabled: bool,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<FormParameterSubtype1Src>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&FormParameterSubtype1> for FormParameterSubtype1 {
    fn from(value: &FormParameterSubtype1) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FormParameterSubtype1Src {
    Null,
    Array(Vec<::serde_json::Value>),
    String(String),
}
impl From<&Self> for FormParameterSubtype1Src {
    fn from(value: &FormParameterSubtype1Src) -> Self {
        value.clone()
    }
}
impl From<Vec<::serde_json::Value>> for FormParameterSubtype1Src {
    fn from(value: Vec<::serde_json::Value>) -> Self {
        Self::Array(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Header {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    #[serde(default)]
    pub disabled: bool,

    pub key: String,

    pub value: String,
}
impl From<&Header> for Header {
    fn from(value: &Header) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum HeaderItem {
    Variant0(Header),
    Variant1(String),
}
impl From<&Self> for HeaderItem {
    fn from(value: &HeaderItem) -> Self {
        value.clone()
    }
}
impl From<Header> for HeaderItem {
    fn from(value: Header) -> Self {
        Self::Variant0(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct HeaderList(pub Vec<Header>);
impl Deref for HeaderList {
    type Target = Vec<Header>;
    fn deref(&self) -> &Vec<Header> {
        &self.0
    }
}
impl From<HeaderList> for Vec<Header> {
    fn from(value: HeaderList) -> Self {
        value.0
    }
}
impl From<&HeaderList> for HeaderList {
    fn from(value: &HeaderList) -> Self {
        value.clone()
    }
}
impl From<Vec<Header>> for HeaderList {
    fn from(value: Vec<Header>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Headers {
    Variant0(Vec<HeaderItem>),
    Variant1(Option<String>),
}
impl From<&Self> for Headers {
    fn from(value: &Headers) -> Self {
        value.clone()
    }
}
impl From<Vec<HeaderItem>> for Headers {
    fn from(value: Vec<HeaderItem>) -> Self {
        Self::Variant0(value)
    }
}
impl From<Option<String>> for Headers {
    fn from(value: Option<String>) -> Self {
        Self::Variant1(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Host {
    String(String),
    Array(Vec<String>),
}
impl From<&Self> for Host {
    fn from(value: &Host) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for Host {
    fn from(value: Vec<String>) -> Self {
        Self::Array(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Info {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    pub name: String,

    #[serde(
        rename = "_postman_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postman_id: Option<String>,

    pub schema: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}
impl From<&Info> for Info {
    fn from(value: &Info) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<EventList>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "protocolProfileBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_profile_behavior: Option<ProtocolProfileBehavior>,
    pub request: Request,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub response: Vec<Response>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableList>,
}
impl From<&Item> for Item {
    fn from(value: &Item) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ItemGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<EventList>,

    pub item: Vec<Items>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "protocolProfileBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_profile_behavior: Option<ProtocolProfileBehavior>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<VariableList>,
}
impl From<&ItemGroup> for ItemGroup {
    fn from(value: &ItemGroup) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Items {
    Variant0(Item),
    Variant1(ItemGroup),
}
impl From<&Self> for Items {
    fn from(value: &Items) -> Self {
        value.clone()
    }
}
impl From<Item> for Items {
    fn from(value: Item) -> Self {
        Self::Variant0(value)
    }
}
impl From<ItemGroup> for Items {
    fn from(value: ItemGroup) -> Self {
        Self::Variant1(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ProtocolProfileBehavior(pub ::serde_json::Map<String, ::serde_json::Value>);
impl Deref for ProtocolProfileBehavior {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}
impl From<ProtocolProfileBehavior> for ::serde_json::Map<String, ::serde_json::Value> {
    fn from(value: ProtocolProfileBehavior) -> Self {
        value.0
    }
}
impl From<&ProtocolProfileBehavior> for ProtocolProfileBehavior {
    fn from(value: &ProtocolProfileBehavior) -> Self {
        value.clone()
    }
}
impl From<::serde_json::Map<String, ::serde_json::Value>> for ProtocolProfileBehavior {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProxyConfig {
    #[serde(default)]
    pub disabled: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    #[serde(rename = "match", default = "defaults::proxy_config_match")]
    pub match_: String,

    #[serde(default = "defaults::default_u64::<u64, 8080>")]
    pub port: u64,

    #[serde(default)]
    pub tunnel: bool,
}
impl From<&ProxyConfig> for ProxyConfig {
    fn from(value: &ProxyConfig) -> Self {
        value.clone()
    }
}
impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            disabled: Default::default(),
            host: Default::default(),
            match_: defaults::proxy_config_match(),
            port: defaults::default_u64::<u64, 8080>(),
            tunnel: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct QueryParam {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,

    #[serde(default)]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&QueryParam> for QueryParam {
    fn from(value: &QueryParam) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Request {
    Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        auth: Option<Auth>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        body: Option<RequestBody>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        certificate: Option<Certificate>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<Description>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        header: Option<RequestHeader>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        method: Option<RequestMethod>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        proxy: Option<ProxyConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<Url>,
    },
    String(String),
}
impl From<&Self> for Request {
    fn from(value: &Request) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RequestBody {
    #[serde(default)]
    pub disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<RequestBodyFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formdata: Vec<FormParameter>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub graphql: ::serde_json::Map<String, ::serde_json::Value>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<RequestBodyMode>,

    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub options: ::serde_json::Map<String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urlencoded: Vec<UrlEncodedParameter>,
}
impl From<&RequestBody> for RequestBody {
    fn from(value: &RequestBody) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RequestBodyFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
}
impl From<&RequestBodyFile> for RequestBodyFile {
    fn from(value: &RequestBodyFile) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RequestBodyMode {
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "urlencoded")]
    Urlencoded,
    #[serde(rename = "formdata")]
    Formdata,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "graphql")]
    Graphql,
}
impl From<&Self> for RequestBodyMode {
    fn from(value: &RequestBodyMode) -> Self {
        *value
    }
}
impl Display for RequestBodyMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Raw => f.write_str("raw"),
            Self::Urlencoded => f.write_str("urlencoded"),
            Self::Formdata => f.write_str("formdata"),
            Self::File => f.write_str("file"),
            Self::Graphql => f.write_str("graphql"),
        }
    }
}
impl FromStr for RequestBodyMode {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, ConversionError> {
        match value {
            "raw" => Ok(Self::Raw),
            "urlencoded" => Ok(Self::Urlencoded),
            "formdata" => Ok(Self::Formdata),
            "file" => Ok(Self::File),
            "graphql" => Ok(Self::Graphql),
            _ => Err("invalid value".into()),
        }
    }
}
impl TryFrom<&str> for RequestBodyMode {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<&String> for RequestBodyMode {
    type Error = ConversionError;
    fn try_from(value: &String) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<String> for RequestBodyMode {
    type Error = ConversionError;
    fn try_from(value: String) -> Result<Self, ConversionError> {
        value.parse()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RequestHeader {
    HeaderList(HeaderList),
    String(String),
}
impl From<&Self> for RequestHeader {
    fn from(value: &RequestHeader) -> Self {
        value.clone()
    }
}
impl From<HeaderList> for RequestHeader {
    fn from(value: HeaderList) -> Self {
        Self::HeaderList(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RequestMethod {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<RequestMethodSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<String>,
}
impl From<&RequestMethod> for RequestMethod {
    fn from(value: &RequestMethod) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestMethodSubtype0 {
    Get,
    Put,
    Post,
    Patch,
    Delete,
    Copy,
    Head,
    Options,
    Link,
    Unlink,
    Purge,
    Lock,
    Unlock,
    Propfind,
    View,
}
impl From<&Self> for RequestMethodSubtype0 {
    fn from(value: &RequestMethodSubtype0) -> Self {
        *value
    }
}
impl Display for RequestMethodSubtype0 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Get => f.write_str("GET"),
            Self::Put => f.write_str("PUT"),
            Self::Post => f.write_str("POST"),
            Self::Patch => f.write_str("PATCH"),
            Self::Delete => f.write_str("DELETE"),
            Self::Copy => f.write_str("COPY"),
            Self::Head => f.write_str("HEAD"),
            Self::Options => f.write_str("OPTIONS"),
            Self::Link => f.write_str("LINK"),
            Self::Unlink => f.write_str("UNLINK"),
            Self::Purge => f.write_str("PURGE"),
            Self::Lock => f.write_str("LOCK"),
            Self::Unlock => f.write_str("UNLOCK"),
            Self::Propfind => f.write_str("PROPFIND"),
            Self::View => f.write_str("VIEW"),
        }
    }
}
impl FromStr for RequestMethodSubtype0 {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, ConversionError> {
        match value {
            "GET" => Ok(Self::Get),
            "PUT" => Ok(Self::Put),
            "POST" => Ok(Self::Post),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "COPY" => Ok(Self::Copy),
            "HEAD" => Ok(Self::Head),
            "OPTIONS" => Ok(Self::Options),
            "LINK" => Ok(Self::Link),
            "UNLINK" => Ok(Self::Unlink),
            "PURGE" => Ok(Self::Purge),
            "LOCK" => Ok(Self::Lock),
            "UNLOCK" => Ok(Self::Unlock),
            "PROPFIND" => Ok(Self::Propfind),
            "VIEW" => Ok(Self::View),
            _ => Err("invalid value".into()),
        }
    }
}
impl TryFrom<&str> for RequestMethodSubtype0 {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<&String> for RequestMethodSubtype0 {
    type Error = ConversionError;
    fn try_from(value: &String) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<String> for RequestMethodSubtype0 {
    type Error = ConversionError;
    fn try_from(value: String) -> Result<Self, ConversionError> {
        value.parse()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cookie: Vec<Cookie>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Headers>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "originalRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_request: Option<Request>,

    #[serde(
        rename = "responseTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_time: Option<ResponseTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timings: Option<::serde_json::Map<String, ::serde_json::Value>>,
}
impl From<&Response> for Response {
    fn from(value: &Response) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ResponseTime {
    Null,
    Number(f64),
    String(String),
}
impl From<&Self> for ResponseTime {
    fn from(value: &ResponseTime) -> Self {
        value.clone()
    }
}
impl From<f64> for ResponseTime {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Script {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ScriptExec>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<Url>,

    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&Script> for Script {
    fn from(value: &Script) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ScriptExec {
    Array(Vec<String>),
    String(String),
}
impl From<&Self> for ScriptExec {
    fn from(value: &ScriptExec) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ScriptExec {
    fn from(value: Vec<String>) -> Self {
        Self::Array(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Url {
    Object {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        hash: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        host: Option<Host>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        path: Option<UrlObjectPath>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        port: Option<String>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        protocol: Option<String>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        query: Vec<QueryParam>,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        raw: Option<String>,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        variable: Vec<Variable>,
    },
    String(String),
}
impl From<&Self> for Url {
    fn from(value: &Url) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UrlEncodedParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Description>,
    #[serde(default)]
    pub disabled: bool,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl From<&UrlEncodedParameter> for UrlEncodedParameter {
    fn from(value: &UrlEncodedParameter) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum UrlObjectPath {
    String(String),
    Array(Vec<UrlObjectPathArrayItem>),
}
impl From<&Self> for UrlObjectPath {
    fn from(value: &UrlObjectPath) -> Self {
        value.clone()
    }
}
impl From<Vec<UrlObjectPathArrayItem>> for UrlObjectPath {
    fn from(value: Vec<UrlObjectPathArrayItem>) -> Self {
        Self::Array(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum UrlObjectPathArrayItem {
    String(String),
    Object {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<String>,
    },
}
impl From<&Self> for UrlObjectPathArrayItem {
    fn from(value: &UrlObjectPathArrayItem) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Variable {
    Variant0(VariableVariant0),
    Variant1(VariableVariant1),
    Variant2(VariableVariant2),
}
impl From<&Self> for Variable {
    fn from(value: &Variable) -> Self {
        value.clone()
    }
}
impl From<VariableVariant0> for Variable {
    fn from(value: VariableVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl From<VariableVariant1> for Variable {
    fn from(value: VariableVariant1) -> Self {
        Self::Variant1(value)
    }
}
impl From<VariableVariant2> for Variable {
    fn from(value: VariableVariant2) -> Self {
        Self::Variant2(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct VariableList(pub Vec<Variable>);
impl Deref for VariableList {
    type Target = Vec<Variable>;
    fn deref(&self) -> &Vec<Variable> {
        &self.0
    }
}
impl From<VariableList> for Vec<Variable> {
    fn from(value: VariableList) -> Self {
        value.0
    }
}
impl From<&VariableList> for VariableList {
    fn from(value: &VariableList) -> Self {
        value.clone()
    }
}
impl From<Vec<Variable>> for VariableList {
    fn from(value: Vec<Variable>) -> Self {
        Self(value)
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub enum VariableVariant0 {}
impl From<&Self> for VariableVariant0 {
    fn from(value: &VariableVariant0) -> Self {
        *value
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub enum VariableVariant1 {}
impl From<&Self> for VariableVariant1 {
    fn from(value: &VariableVariant1) -> Self {
        *value
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub enum VariableVariant2 {}
impl From<&Self> for VariableVariant2 {
    fn from(value: &VariableVariant2) -> Self {
        *value
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Version {
    Object {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        identifier: Option<VersionObjectIdentifier>,

        major: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        meta: Option<::serde_json::Value>,

        minor: u64,

        patch: u64,
    },
    String(String),
}
impl From<&Self> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}

#[derive(Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VersionObjectIdentifier(String);
impl Deref for VersionObjectIdentifier {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<VersionObjectIdentifier> for String {
    fn from(value: VersionObjectIdentifier) -> Self {
        value.0
    }
}
impl TryFrom<&VersionObjectIdentifier> for VersionObjectIdentifier {
    type Error = ();

    fn try_from(value: &VersionObjectIdentifier) -> Result<Self, Self::Error> {
        Ok(value.clone())
    }
}
impl FromStr for VersionObjectIdentifier {
    type Err = ConversionError;
    fn from_str(value: &str) -> Result<Self, ConversionError> {
        if value.chars().count() > 10usize {
            return Err("longer than 10 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl TryFrom<&str> for VersionObjectIdentifier {
    type Error = ConversionError;
    fn try_from(value: &str) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<&String> for VersionObjectIdentifier {
    type Error = ConversionError;
    fn try_from(value: &String) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl TryFrom<String> for VersionObjectIdentifier {
    type Error = ConversionError;
    fn try_from(value: String) -> Result<Self, ConversionError> {
        value.parse()
    }
}
impl<'de> Deserialize<'de> for VersionObjectIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: ConversionError| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}

pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn proxy_config_match() -> ::std::string::String {
        "http+https://*/*".to_string()
    }
}
