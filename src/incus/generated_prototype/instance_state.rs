// Auto-generated. Do not edit.

use crate::incus::InstanceStateCPU;
use crate::incus::InstanceStateDisk;
use crate::incus::InstanceStateMemory;
use crate::incus::InstanceStateNetwork;
use crate::incus::InstanceStateOSInfo;
use crate::incus::StatusCode;
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceState {
    pub status: String,
    pub status_code: StatusCode,
    pub disk: HashMap<String, InstanceStateDisk>,
    pub memory: InstanceStateMemory,
    pub network: HashMap<String, InstanceStateNetwork>,
    pub pid: i64,
    pub processes: i64,
    pub cpu: InstanceStateCPU,
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_info: Option<InstanceStateOSInfo>,
}
