// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePoolPut {
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub description: String,
}
