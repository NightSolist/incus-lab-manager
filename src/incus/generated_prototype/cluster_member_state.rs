// Auto-generated. Do not edit.

use crate::incus::ClusterMemberSysInfo;
use crate::incus::StoragePoolState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterMemberState {
    #[serde(rename = "sysinfo")]
    pub sys_info: ClusterMemberSysInfo,
    pub storage_pools: HashMap<String, StoragePoolState>,
}
