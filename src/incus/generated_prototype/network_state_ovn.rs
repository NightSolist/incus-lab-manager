// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStateOVN {
    pub chassis: String,
    pub logical_router: String,
    pub logical_switch: String,
    #[serde(rename = "uplink_ipv4")]
    pub uplink_i_pv4: String,
    #[serde(rename = "uplink_ipv6")]
    pub uplink_i_pv6: String,
}
