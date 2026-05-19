// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesGPUCardDRM {
    pub id: u64,
    pub card_name: String,
    pub card_device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_device: Option<String>,
}
