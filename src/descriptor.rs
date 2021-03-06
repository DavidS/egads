use reqwest_middleware::ClientWithMiddleware;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, instrument};

use crate::{
    list::DirectoryItem, Error, IconKey, ParameterFormat, ParameterType, Ref, RestDescriptionKind,
    RestProtocol, Result, Version,
};
// {
//   "kind": "discovery#restDescription",
//   "discoveryVersion": "v1",
//   "id": string,
//   "name": string,
//   "canonicalName": string,
//   "version": string,
//   "revision": string,
//   "title": string,
//   "description": string,
//   "icons": {
//     "x16": string,
//     "x32": string
//   },
//   "documentationLink": string,
//   "labels": [
//     string
//   ],
//   "protocol": "rest",
//   "baseUrl": string,
//   "basePath": string,
//   "rootUrl": string,
//   "servicePath": string,
//   "batchPath": "batch",
//   "parameters": {
//     (key): {
//       "id": string,
//       "type": string,
//       "$ref": string,
//       "description": string,
//       "default": string,
//       "required": boolean,
//       "format": string,
//       "pattern": string,
//       "minimum": string,
//       "maximum": string,
//       "enum": [
//         string
//       ],
//       "enumDescriptions": [
//         string
//       ],
//       "repeated": boolean,
//       "location": string,
//       "properties": {
//         (key): (JsonSchema)
//       },
//       "additionalProperties": (JsonSchema),
//       "items": (JsonSchema),
//       "annotations": {
//         "required": [
//           string
//         ]
//       }
//     }
//   },
//   "auth": {
//     "oauth2": {
//       "scopes": {
//         (key): {
//           "description": string
//         }
//       }
//     }
//   },
//   "features": [
//     string
//   ],
//   "schemas": {
//     (key): {
//       "id": string,
//       "type": string,
//       "$ref": string,
//       "description": string,
//       "default": string,
//       "required": boolean,
//       "format": string,
//       "pattern": string,
//       "minimum": string,
//       "maximum": string,
//       "enum": [
//         string
//       ],
//       "enumDescriptions": [
//         string
//       ],
//       "repeated": boolean,
//       "location": string,
//       "properties": {
//         (key): (JsonSchema)
//       },
//       "additionalProperties": (JsonSchema),
//       "items": (JsonSchema),
//       "annotations": {
//         "required": [
//           string
//         ]
//       }
//     }
//   },
//   "methods": {
//     (key): {
//       "id": string,
//       "path": string,
//       "httpMethod": string,
//       "description": string,
//       "parameters": {
//         (key): {
//           "id": string,
//           "type": string,
//           "$ref": string,
//           "description": string,
//           "default": string,
//           "required": boolean,
//           "format": string,
//           "pattern": string,
//           "minimum": string,
//           "maximum": string,
//           "enum": [
//             string
//           ],
//           "enumDescriptions": [
//             string
//           ],
//           "repeated": boolean,
//           "location": string,
//           "properties": {
//             (key): (JsonSchema)
//           },
//           "additionalProperties": (JsonSchema),
//           "items": (JsonSchema),
//           "annotations": {
//             "required": [
//               string
//             ]
//           }
//         }
//       },
//       "parameterOrder": [
//         string
//       ],
//       "request": {
//         "$ref": string
//       },
//       "response": {
//         "$ref": string
//       },
//       "scopes": [
//         (value)
//       ],
//       "supportsMediaDownload": boolean,
//       "supportsMediaUpload": boolean,
//       "mediaUpload": {
//         "accept": [
//           string
//         ],
//         "maxSize": string,
//         "protocols": {
//           "simple": {
//             "multipart": true,
//             "path": string
//           },
//           "resumable": {
//             "multipart": true,
//             "path": string
//           }
//         }
//       },
//       "supportsSubscription": boolean
//     }
//   },
//   "resources": {
//     (key): {
//       "methods": {
//         (key): {
//           "id": string,
//           "path": string,
//           "httpMethod": string,
//           "description": string,
//           "parameters": {
//             (key): {
//               "id": string,
//               "type": string,
//               "$ref": string,
//               "description": string,
//               "default": string,
//               "required": boolean,
//               "format": string,
//               "pattern": string,
//               "minimum": string,
//               "maximum": string,
//               "enum": [
//                 string
//               ],
//               "enumDescriptions": [
//                 string
//               ],
//               "repeated": boolean,
//               "location": string,
//               "properties": {
//                 (key): (JsonSchema)
//               },
//               "additionalProperties": (JsonSchema),
//               "items": (JsonSchema),
//               "annotations": {
//                 "required": [
//                   string
//                 ]
//               }
//             }
//           },
//           "parameterOrder": [
//             string
//           ],
//           "request": {
//             "$ref": string
//           },
//           "response": {
//             "$ref": string
//           },
//           "scopes": [
//             (value)
//           ],
//           "supportsMediaDownload": boolean,
//           "supportsMediaUpload": boolean,
//           "mediaUpload": {
//             "accept": [
//               string
//             ],
//             "maxSize": string,
//             "protocols": {
//               "simple": {
//                 "multipart": true,
//                 "path": string
//               },
//               "resumable": {
//                 "multipart": true,
//                 "path": string
//               }
//             }
//           },
//           "supportsSubscription": boolean
//         }
//       },
//       "resources": {
//         (key): (RestResource)
//       }
//     }
//   }
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RestDescription {
    pub kind: RestDescriptionKind,
    pub discovery_version: Version,
    pub id: String,
    pub name: String,
    pub canonical_name: Option<String>,
    pub version: String,
    pub revision: String,
    pub title: String,
    pub description: String,
    pub icons: HashMap<IconKey, String>,
    pub documentation_link: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub protocol: RestProtocol,
    pub base_url: String,
    pub base_path: String,
    pub root_url: String,
    pub service_path: String,
    pub batch_path: String,
    pub parameters: HashMap<String, Parameter>,
    pub auth: Option<Auth>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub schemas: HashMap<String, Parameter>,
    #[serde(default)]
    pub methods: HashMap<String, Method>,
    #[serde(default)]
    pub resources: HashMap<String, Resource>,

    //region other random fields found in actual descriptors
    pub fully_encode_reserved_expansion: Option<bool>,
    pub mtls_root_url: Option<String>,
    pub owner_domain: Option<String>,
    pub owner_name: Option<String>,
    #[serde(rename = "version_module")]
    pub version_module: Option<bool>,
    pub package_path: Option<String>,
    pub etag: Option<String>,
    //endregion
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<ParameterType>,

    pub description: Option<String>,
    pub default: Option<String>,
    #[serde(default)]
    pub required: bool,
    pub format: Option<ParameterFormat>,
    pub pattern: Option<String>,
    pub minimum: Option<String>,
    pub maximum: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub enum_descriptions: Option<Vec<String>>,
    #[serde(default)]
    pub repeated: Option<bool>,
    pub location: Option<String>,
    #[serde(default)]
    pub properties: HashMap<String, Value>, // TODO: JsonSchema
    #[serde(default)]
    pub additional_properties: Value, // TODO: JsonSchema
    #[serde(default)]
    pub items: Value, // TODO: JsonSchema
    pub annotations: Option<Annotations>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ParameterOrRef {
    #[serde(flatten)]
    pub parameter: Parameter,
    #[serde(flatten)]
    pub ref_: Ref,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Annotations {
    pub required: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Auth {
    pub oauth2: OAuth2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuth2 {
    pub scopes: HashMap<OAuth2Scope, OAuth2ScopeDesciption>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct OAuth2Scope(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuth2ScopeDesciption {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Method {
    pub id: String,
    pub path: String,
    pub http_method: String, // TODO: add a enum for this
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: HashMap<String, ParameterOrRef>,
    #[serde(default)]
    pub parameter_order: Vec<String>,
    pub request: Option<Ref>,
    pub response: Option<Ref>,
    #[serde(default)]
    pub scopes: Vec<OAuth2Scope>,
    pub supports_media_download: Option<bool>,
    pub supports_media_upload: Option<bool>,
    #[serde(default)]
    pub media_upload: HashMap<String, Value>, //TODO
    pub supports_subscription: Option<bool>,

    //region other random fields found in actual descriptors
    pub flat_path: Option<String>,
    pub use_media_download_service: Option<bool>,
    pub streaming_type: Option<String>,
    //endregion
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Resource {
    #[serde(default)]
    pub methods: HashMap<String, Method>,
    #[serde(default)]
    pub resources: HashMap<String, Resource>,
}

pub async fn fetch_item(
    client: &ClientWithMiddleware,
    item: &DirectoryItem,
) -> Result<RestDescription> {
    fetch_url(client, &item.discovery_rest_url).await
}

#[instrument]
pub async fn fetch_url(
    client: &ClientWithMiddleware,
    discovery_rest_url: &str,
) -> Result<RestDescription> {
    // debug!("Build descriptor fetcher");
    // let client = build_fetcher();
    debug!("Starting descriptor fetch");
    let response = client.get(discovery_rest_url).send().await?;
    debug!("Fetching descriptor response");
    let body = response.text().await?;
    debug!("Parsing response");

    from_str(body)
}

pub fn from_str(json: String) -> Result<RestDescription> {
    debug!("Starting descriptor parse");
    serde_json::from_str(&json).map_err(|source| Error::JsonError {
        message: "couldn't parse descriptor".into(),
        json,
        source,
    })
}
