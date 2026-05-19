// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use crate::incus::ImageMetadataTemplate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageMetadata {
    pub architecture: String,
    pub creation_date: i64,
    pub expiry_date: i64,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub properties: ConfigMap,
    pub templates: HashMap<String, Option<ImageMetadataTemplate>>,
}
