// Auto-generated. Do not edit.

use crate::incus::Instance;
use crate::incus::InstanceBackup;
use crate::incus::InstanceSnapshot;
use crate::incus::InstanceState;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceFull {
    #[serde(flatten)]
    pub instance: Instance,
    pub backups: Vec<InstanceBackup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
    pub snapshots: Vec<InstanceSnapshot>,
}
