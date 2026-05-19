// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStateBridge {
    pub id: String,
    pub stp: bool,
    pub forward_delay: u64,
    pub vlan_default: u64,
    pub vlan_filtering: bool,
    pub upper_devices: Vec<String>,
}
