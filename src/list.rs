use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub enum DiscoveryListKind {
    #[serde(rename = "discovery#directoryList")]
    DirectoryList,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiscoveryItemKind {
    #[serde(rename = "discovery#directoryItem")]
    DirectoryItem,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum IconKey {
    #[serde(rename = "x16")]
    X16,
    #[serde(rename = "x32")]
    X32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DirectoryList {
    pub discovery_version: String,
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

pub async fn fetch_current_list(
    client: Client,
    name: Option<&str>,
    preferred: bool,
) -> reqwest::Result<DirectoryList> {
    let mut request = client.get(LIST_URL);

    if let Some(name) = name {
        request = request.query(&[("name", name)]);
    }
    if preferred {
        request = request.query(&[("preferred", "true")]);
    }
    let response = request.send().await?.text().await?;

    let result = serde_json::from_str(&response).expect("couldn't parse service list");

    return Ok(result);
}
