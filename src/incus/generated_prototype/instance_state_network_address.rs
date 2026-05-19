// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceStateNetworkAddress {
    pub family: String,
    pub address: String,
    pub netmask: String,
    pub scope: String,
}
