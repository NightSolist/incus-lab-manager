// Auto-generated. Do not edit.

use crate::incus::Cluster;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterPut {
    #[serde(flatten)]
    pub cluster: Cluster,
    pub cluster_address: String,
    pub cluster_certificate: String,
    pub server_address: String,
    pub cluster_token: String,
}
