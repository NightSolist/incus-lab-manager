// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPut {
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tags: Option<HashMap<String, Vec<String>>>,
}
