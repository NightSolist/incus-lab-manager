// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImagePut {
    pub auto_update: bool,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub properties: ConfigMap,
    pub public: bool,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub profiles: Vec<String>,
}
