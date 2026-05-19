// Auto-generated. Do not edit.

use crate::incus::ResourcesCPUSocket;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesCPU {
    pub architecture: String,
    pub sockets: Vec<ResourcesCPUSocket>,
    pub total: u64,
}
