// Auto-generated. Do not edit.

use crate::incus::ResourcesMemoryNode;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ResourcesMemoryNode>>,
    pub hugepages_total: u64,
    pub hugepages_used: u64,
    pub hugepages_size: u64,
    pub used: u64,
    pub total: u64,
}
