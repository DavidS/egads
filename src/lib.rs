use serde::{Deserialize, Serialize};

pub mod descriptor;
pub mod list;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum DiscoveryListKind {
    #[serde(rename = "discovery#directoryList")]
    DirectoryList,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum DiscoveryItemKind {
    #[serde(rename = "discovery#directoryItem")]
    DirectoryItem,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum RestDescriptionKind {
    #[serde(rename = "discovery#restDescription")]
    RestDescription,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum Version {
    #[serde(rename = "v1")]
    V1,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum IconKey {
    #[serde(rename = "x16")]
    X16,
    #[serde(rename = "x32")]
    X32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum RestProtocol {
    #[serde(rename = "rest")]
    REST,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum BatchPath {
    #[serde(rename = "batch")]
    Batch,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Array,
    Boolean,
    Integer,
    Number,
    String,
    Object,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ParameterFormat {
    GoogleDatetime,
    Int32,
    Uint32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}
