// Auto-generated. Do not edit.

use crate::incus::ClusterGroupPost;
use crate::incus::ClusterGroupPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterGroup {
    #[serde(flatten)]
    pub cluster_group_put: ClusterGroupPut,
    #[serde(flatten)]
    pub cluster_group_post: ClusterGroupPost,
    pub used_by: Vec<String>,
}
