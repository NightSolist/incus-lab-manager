// Auto-generated. Do not edit.

use crate::incus::ClusterMemberPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterMember {
    #[serde(flatten)]
    pub cluster_member_put: ClusterMemberPut,
    pub server_name: String,
    pub url: String,
    pub database: bool,
    pub status: String,
    pub message: String,
    pub architecture: String,
}
