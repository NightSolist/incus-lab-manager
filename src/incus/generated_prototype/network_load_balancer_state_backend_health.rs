// Auto-generated. Do not edit.

use crate::incus::NetworkLoadBalancerStateBackendHealthPort;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancerStateBackendHealth {
    pub address: String,
    pub ports: Vec<NetworkLoadBalancerStateBackendHealthPort>,
}
