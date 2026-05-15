// Auto-generated. Do not edit.

use serde::{Serialize, Deserialize};
use crate::incus::ConfigMap;
use crate::incus::DevicesMap;
use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::devices_map::{deserialize_devices_map, deserialize_option_devices_map};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilePut {    #[serde(deserialize_with = "deserialize_config_map")]    pub config: ConfigMap,    pub description: String,    #[serde(deserialize_with = "deserialize_devices_map")]    pub devices: DevicesMap,}