// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use crate::incus::ServerStorageDriverInfo;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerEnvironment {
    pub addresses: Vec<String>,
    pub architectures: Vec<String>,
    pub certificate: String,
    pub certificate_fingerprint: String,
    pub driver: String,
    pub driver_version: String,
    pub firewall: String,
    pub kernel: String,
    pub kernel_architecture: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub kernel_features: ConfigMap,
    pub kernel_version: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub lxc_features: ConfigMap,
    pub os_name: String,
    pub os_version: String,
    pub project: String,
    pub server: String,
    pub server_clustered: bool,
    pub server_event_mode: String,
    pub server_name: String,
    pub server_pid: i64,
    pub server_version: String,
    pub storage: String,
    pub storage_version: String,
    pub storage_supported_drivers: Vec<ServerStorageDriverInfo>,
}
