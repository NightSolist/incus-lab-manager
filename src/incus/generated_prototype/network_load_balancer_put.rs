// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use crate::incus::NetworkLoadBalancerBackend;
use crate::incus::NetworkLoadBalancerPort;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLoadBalancerPut {
    pub description: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub backends: Vec<NetworkLoadBalancerBackend>,
    pub ports: Vec<NetworkLoadBalancerPort>,
}
