// Auto-generated. Do not edit.

use crate::incus::NetworkLoadBalancerPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancersPost {
    #[serde(flatten)]
    pub network_load_balancer_put: NetworkLoadBalancerPut,
    pub listen_address: String,
}
