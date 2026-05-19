// Auto-generated. Do not edit.

use crate::incus::ResourcesNetworkCardPortInfiniband;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesNetworkCardPort {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub port: u64,
    pub protocol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_modes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transceiver_type: Option<String>,
    pub auto_negotiation: bool,
    pub link_detected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_speed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_duplex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infiniband: Option<ResourcesNetworkCardPortInfiniband>,
}
