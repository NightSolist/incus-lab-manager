// Auto-generated. Do not edit.

use crate::incus::InstanceStateNetworkAddress;
use crate::incus::InstanceStateNetworkCounters;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceStateNetwork {
    pub addresses: Vec<InstanceStateNetworkAddress>,
    pub counters: InstanceStateNetworkCounters,
    pub hwaddr: String,
    pub host_name: String,
    pub mtu: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
