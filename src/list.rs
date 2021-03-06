use crate::{DiscoveryItemKind, DiscoveryListKind, Error, IconKey, Result, Version};
use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::instrument;

// {
//  "kind": "discovery#directoryList",
//  "discoveryVersion": "v1",
//  "items": [
//    {
//      "kind": "discovery#directoryItem",
//      "id": string,
//      "name": string,
//      "version": string,
//      "title": string,
//      "description": string,
//      "discoveryRestUrl": string,
//      "discoveryLink": string,
//      "icons": {
//        "x16": string,
//        "x32": string
//      },
//      "documentationLink": string,
//      "labels": [
//        string
//      ],
//      "preferred": boolean
//    }
//  ]
//}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DirectoryList {
    pub discovery_version: Version,
    pub kind: DiscoveryListKind,
    pub items: Vec<DirectoryItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DirectoryItem {
    pub kind: DiscoveryItemKind,
    pub id: String,
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub discovery_rest_url: String,
    pub discovery_link: Option<String>,
    pub icons: HashMap<IconKey, String>,
    pub documentation_link: Option<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub preferred: bool,
}

pub(crate) const LIST_URL: &str = "https://discovery.googleapis.com/discovery/v1/apis";

/// Fetch a list of all discoverable APIs.
/// ```
/// fetch(client);
/// ```
pub async fn fetch(client: &ClientWithMiddleware) -> Result<DirectoryList> {
    fetch_impl(client, None, false).await
}

/// Fetch a specific API designated by the given name.
pub async fn fetch_specific(client: &ClientWithMiddleware, name: &str) -> Result<DirectoryList> {
    fetch_impl(client, Some(name), false).await
}

/// Fetch the preferred API version designated by the given name.
pub async fn fetch_preferred(client: &ClientWithMiddleware, name: &str) -> Result<DirectoryList> {
    fetch_impl(client, Some(name), true).await
}

#[instrument]
async fn fetch_impl(
    client: &ClientWithMiddleware,
    name: Option<&str>,
    preferred: bool,
) -> Result<DirectoryList> {
    let mut request = client.get(LIST_URL);

    if let Some(name) = name {
        request = request.query(&[("name", name)]);
    }
    if preferred {
        request = request.query(&[("preferred", "true")]);
    }
    let response = request.send().await?;

    let body = response.text().await?;

    from_str(body)
}

pub fn from_str(json: String) -> Result<DirectoryList> {
    serde_json::from_str(&json).map_err(|source| Error::JsonError {
        message: "couldn't parse service list".into(),
        json,
        source,
    })
}
