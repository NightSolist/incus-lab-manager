// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLogging {
    pub message: String,
    pub level: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub context: ConfigMap,
}
