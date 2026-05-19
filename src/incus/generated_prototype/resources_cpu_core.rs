// Auto-generated. Do not edit.

use crate::incus::ResourcesCPUThread;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesCPUCore {
    pub core: u64,
    pub die: u64,
    pub threads: Vec<ResourcesCPUThread>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u64>,
    pub flags: Vec<String>,
}
