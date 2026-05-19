// Auto-generated. Do not edit.

use crate::incus::ResourcesCPUAddressSizes;
use crate::incus::ResourcesCPUCache;
use crate::incus::ResourcesCPUCore;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesCPUSocket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    pub socket: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<Vec<ResourcesCPUCache>>,
    pub cores: Vec<ResourcesCPUCore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_minimum: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_turbo: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_sizes: Option<ResourcesCPUAddressSizes>,
}
