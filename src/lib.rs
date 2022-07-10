//! egads! is a simple library to work with [Google API Discovery Service](https://developers.google.com/discovery) definitions.
//!
//! This crate is split into the two major operations on the discovery service:
//! * [`list`] services, and
//! * fetch a service [`descriptor`].
//!
//! Each module contains functions to fetch service lists or descriptors,
//! as well as a `from_str` function to parse any JSON into objects.

use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

pub mod descriptor;
pub(crate) mod fetcher;
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
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    Any,
    Array,
    Boolean,
    Integer,
    Number,
    Object,
    String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ParameterFormat {
    Int32,
    Uint32,
    Double,
    Float,
    Byte,
    Date,
    DateTime,
    GoogleDatetime,
    GoogleDuration,
    GoogleFieldmask,
    Int64,
    Uint64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that may occur while interacting with the discovery API
#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("couldn't send request")]
    HttpSendError(#[from] reqwest_middleware::Error),
    #[error("couldn't receive response")]
    HttpReceiveError(#[from] reqwest::Error),
    #[error("json error")]
    JsonError {
        message: String,
        json: String,
        source: serde_json::Error,
    },
}
