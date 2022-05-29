use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ApiDescriptor {
    pub auth: Auth,
    pub base_path: String,
    pub base_url: String,
    pub batch_path: String,
    pub canonical_name: String,
    pub description: String,
    pub discovery_version: String,
    pub documentation_link: String,
    pub fully_encode_reserved_expansion: bool,
    pub icons: HashMap<String, String>,
    pub id: String,
    pub kind: String,
    pub mtls_root_url: String,
    pub name: String,
    pub owner_domain: String,
    pub owner_name: String,
    pub parameters: HashMap<String, ParameterInfo>,
    pub protocol: String,
    pub resources: HashMap<String, ResourceInfo>,
    pub revision: String,
    pub root_url: String,
    pub schemas: HashMap<String, SchemaInfo>,
    pub service_path: String,
    pub title: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub oauth2: OAuth2,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuth2 {
    pub scopes: HashMap<OAuth2Scope, OAuth2ScopeDesciptor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct OAuth2Scope(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuth2ScopeDesciptor {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceInfo {
    #[serde(default)]
    pub methods: HashMap<String, MethodInfo>,
    #[serde(default)]
    pub resources: HashMap<String, ResourceInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MethodInfo {
    #[serde(default)]
    pub parameters: HashMap<String, ParameterInfo>,
    #[serde(default)]
    pub scopes: Vec<OAuth2Scope>,
    pub http_method: String,
    pub parameter_order: Vec<String>,
    #[serde(default)]
    pub request: Option<Value>,
    #[serde(default)]
    pub response: Option<Value>,
    pub flat_path: String,
    pub path: String,
    pub id: String,
    pub description: String,
    pub supports_media_upload: Option<bool>,
    pub supports_media_download: Option<bool>,
    pub use_media_download_service: Option<bool>,
    #[serde(default)]
    pub media_upload: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParameterInfo {
    pub location: Value,
    #[serde(rename = "type")]
    pub type_: ParameterType,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub enum_descriptions: Option<Vec<String>>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub format: Option<ParameterFormat>,
    pub minimum: Option<Value>,
    pub maximum: Option<Value>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub repeated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Array,
    Boolean,
    Integer,
    Number,
    String,
    Object,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ParameterFormat {
    GoogleDatetime,
    Int32,
    Uint32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaInfo {
    pub id: String,
    pub properties: HashMap<String, PropertyInfo>,
    #[serde(rename = "type")]
    pub type_: ParameterType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyInfo {
    pub description: Option<String>,
    pub default: Option<Value>,
    #[serde(rename = "type")]
    pub type_: Option<ParameterType>,
}
