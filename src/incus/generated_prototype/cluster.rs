// Auto-generated. Do not edit.

use crate::incus::ClusterMemberConfigKey;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cluster {
    pub server_name: String,
    pub enabled: bool,
    pub member_config: Vec<ClusterMemberConfigKey>,
}
