// Auto-generated. Do not edit.

use crate::incus::NetworkLoadBalancerStateBackendHealth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancerState {
    pub backend_health: HashMap<String, NetworkLoadBalancerStateBackendHealth>,
}
