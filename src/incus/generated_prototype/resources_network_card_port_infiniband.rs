// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesNetworkCardPortInfiniband {
    #[serde(rename = "issm_name", skip_serializing_if = "Option::is_none")]
    pub is_sm_name: Option<String>,
    #[serde(rename = "issm_device", skip_serializing_if = "Option::is_none")]
    pub is_sm_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mad_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mad_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb_device: Option<String>,
}
