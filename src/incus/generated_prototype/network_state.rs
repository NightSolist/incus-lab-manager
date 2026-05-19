// Auto-generated. Do not edit.

use crate::incus::NetworkStateAddress;
use crate::incus::NetworkStateBond;
use crate::incus::NetworkStateBridge;
use crate::incus::NetworkStateCounters;
use crate::incus::NetworkStateOVN;
use crate::incus::NetworkStateVLAN;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkState {
    pub addresses: Vec<NetworkStateAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<NetworkStateCounters>,
    pub hwaddr: String,
    pub mtu: i64,
    pub state: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bond: Option<NetworkStateBond>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<NetworkStateBridge>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<NetworkStateVLAN>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ovn: Option<NetworkStateOVN>,
}
