// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::devices_map::{deserialize_devices_map, deserialize_option_devices_map};
use crate::incus::ConfigMap;
use crate::incus::DevicesMap;
use crate::incus::InstanceSnapshotPut;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceSnapshot {
    #[serde(flatten)]
    pub instance_snapshot_put: InstanceSnapshotPut,
    pub architecture: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    #[serde(deserialize_with = "deserialize_devices_map")]
    pub devices: DevicesMap,
    pub ephemeral: bool,
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
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub profiles: Vec<String>,
    pub stateful: bool,
    pub size: i64,
}
