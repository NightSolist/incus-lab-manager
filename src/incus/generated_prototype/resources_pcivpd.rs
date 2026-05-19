// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesPCIVPD {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_option_config_map",
        skip_serializing_if = "Option::is_none"
    )]
    pub entries: Option<ConfigMap>,
}
