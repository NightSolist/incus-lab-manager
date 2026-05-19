// Auto-generated. Do not edit.

use crate::incus::NetworkPeerPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPeersPost {
    #[serde(flatten)]
    pub network_peer_put: NetworkPeerPut,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_network: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_integration: Option<String>,
}
