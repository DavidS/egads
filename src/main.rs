use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs::File, io::BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct ApiDescriptor {
    auth: Auth,
    base_path: String,
    base_url: String,
    batch_path: String,
    canonical_name: String,
    description: String,
    discovery_version: String,
    documentation_link: String,
    fully_encode_reserved_expansion: bool,
    icons: HashMap<String, String>,
    id: String,
    kind: String,
    mtls_root_url: String,
    name: String,
    owner_domain: String,
    owner_name: String,
    parameters: HashMap<String, ParameterInfo>,
    protocol: String,
    resources: HashMap<String, ResourceInfo>,
    revision: String,
    root_url: String,
    schemas: HashMap<String, SchemaInfo>,
    service_path: String,
    title: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Auth {
    oauth2: OAuth2,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAuth2 {
    scopes: HashMap<OAuth2Scope, OAuth2ScopeDesciptor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct OAuth2Scope(String);

#[derive(Serialize, Deserialize, Debug)]
struct OAuth2ScopeDesciptor {
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResourceInfo {
    #[serde(default)]
    methods: HashMap<String, MethodInfo>,
    #[serde(default)]
    resources: HashMap<String, ResourceInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct MethodInfo {
    #[serde(default)]
    parameters: HashMap<String, ParameterInfo>,
    #[serde(default)]
    scopes: Vec<OAuth2Scope>,
    http_method: String,
    parameter_order: Vec<String>,
    #[serde(default)]
    request: Option<Value>,
    #[serde(default)]
    response: Option<Value>,
    flat_path: String,
    path: String,
    id: String,
    description: String,
    supports_media_upload: Option<bool>,
    supports_media_download: Option<bool>,
    use_media_download_service: Option<bool>,
    #[serde(default)]
    media_upload: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct ParameterInfo {
    location: Value,
    #[serde(rename = "type")]
    type_: ParameterType,
    description: Option<String>,
    default: Option<Value>,
    enum_descriptions: Option<Vec<String>>,
    #[serde(rename = "enum")]
    enum_: Option<Vec<String>>,
    format: Option<ParameterFormat>,
    minimum: Option<Value>,
    maximum: Option<Value>,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    repeated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ParameterType {
    Array,
    Boolean,
    Integer,
    Number,
    String,
    Object,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum ParameterFormat {
    GoogleDatetime,
    Int32,
    Uint32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SchemaInfo {
    id: String,
    properties: HashMap<String, PropertyInfo>,
    #[serde(rename = "type")]
    type_: ParameterType,
}
#[derive(Serialize, Deserialize, Debug)]
struct PropertyInfo {
    description: Option<String>,
    default: Option<Value>,
    #[serde(rename = "type")]
    type_: Option<ParameterType>,
}

fn main() {
    println!("Hello, world!");
    let args = Cli::parse();
    println!("{:#?}", args.path);

    let file = File::open(&args.path).expect("could not open file");
    let reader = BufReader::new(file);
    let api: ApiDescriptor =
        serde_json::from_reader(reader).expect("couldn't parse service description");

    // println!("{:#?}", api.resources["subscriptions"].methods["list"]);
    println!("{:#?}", api.schemas["SubscriptionListResponse"]);
}
