// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesGPUCardNvidia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuda_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvrm_version: Option<String>,
    pub brand: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    pub card_name: String,
    pub card_device: String,
}
