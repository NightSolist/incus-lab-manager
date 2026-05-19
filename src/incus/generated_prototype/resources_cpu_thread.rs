// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesCPUThread {
    pub id: i64,
    pub numa_node: u64,
    pub thread: u64,
    pub online: bool,
    pub isolated: bool,
}
