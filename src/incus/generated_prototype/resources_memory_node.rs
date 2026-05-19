// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesMemoryNode {
    pub numa_node: u64,
    pub hugepages_used: u64,
    pub hugepages_total: u64,
    pub used: u64,
    pub total: u64,
}
