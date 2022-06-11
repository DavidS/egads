//! egads! is a simple library to work with [Google API Discovery Service](https://developers.google.com/discovery) definitions.
//!
//! This crate is split into the two major operations on the discovery service:
//! * [`list`] services, and
//! * fetch a service [`descriptor`].
//!
//! Each module contains functions to fetch service lists or descriptors,
//! as well as a `from_str` function to parse any JSON into objects.

use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt};

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
pub struct Error {
    message: String, //TODO: move to Box<Inner> model to reduce size of Error being passed around
    source: Option<Box<dyn StdError>>,
}

impl Error {
    pub(crate) fn new<E>(message: &str, source: Option<E>) -> Error
    where
        E: Into<Box<dyn StdError>>,
    {
        Error {
            message: message.into(),
            source: source.map(Into::into),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("egads::Error");

        builder.field("message", &self.message);

        if let Some(ref source) = self.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Egads! {}", self.message)?;
        if let Some(e) = &self.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}
