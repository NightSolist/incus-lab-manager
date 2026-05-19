// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolumeSource {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub pool: String,
    pub certificate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(
        rename = "secrets",
        default,
        deserialize_with = "deserialize_option_config_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub websockets: Option<ConfigMap>,
    pub volume_only: bool,
    pub refresh: bool,
    pub refresh_exclude_older: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    pub location: String,
}
