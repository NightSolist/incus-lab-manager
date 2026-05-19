// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancerBackend {
    pub name: String,
    pub description: String,
    pub target_port: String,
    pub target_address: String,
}
