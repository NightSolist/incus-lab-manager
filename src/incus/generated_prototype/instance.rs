// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::devices_map::{deserialize_devices_map, deserialize_option_devices_map};
use crate::incus::ConfigMap;
use crate::incus::DevicesMap;
use crate::incus::InstancePut;
use crate::incus::StatusCode;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Instance {
    #[serde(flatten)]
    pub instance_put: InstancePut,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_config_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub expanded_config: Option<ConfigMap>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_devices_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub expanded_devices: Option<DevicesMap>,
    pub name: String,
    pub status: String,
    pub status_code: StatusCode,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub location: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub project: String,
}
