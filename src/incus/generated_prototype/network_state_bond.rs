// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStateBond {
    pub mode: String,
    pub transmit_policy: String,
    pub up_delay: u64,
    pub down_delay: u64,
    pub mii_frequency: u64,
    pub mii_state: String,
    pub lower_devices: Vec<String>,
}
