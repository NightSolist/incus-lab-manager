// Auto-generated. Do not edit.

use crate::incus::ClusterGroupPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterGroupsPost {
    #[serde(flatten)]
    pub cluster_group_put: ClusterGroupPut,
    pub name: String,
}
