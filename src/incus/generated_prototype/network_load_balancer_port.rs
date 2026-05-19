// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancerPort {
    pub description: String,
    pub protocol: String,
    pub listen_port: String,
    pub target_backend: Vec<String>,
}
