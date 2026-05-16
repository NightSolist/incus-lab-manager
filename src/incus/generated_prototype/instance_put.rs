// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::devices_map::{deserialize_devices_map, deserialize_option_devices_map};
use crate::incus::ConfigMap;
use crate::incus::DevicesMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstancePut {
    pub architecture: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    #[serde(deserialize_with = "deserialize_devices_map")]
    pub devices: DevicesMap,
    pub ephemeral: bool,
    pub profiles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_only: Option<bool>,
    pub stateful: bool,
    pub description: String,
}
