// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use crate::incus::NetworkForwardPort;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkForwardPut {
    pub description: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub ports: Vec<NetworkForwardPort>,
}
