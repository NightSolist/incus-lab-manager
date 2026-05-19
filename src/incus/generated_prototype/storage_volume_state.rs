// Auto-generated. Do not edit.

use crate::incus::StorageVolumeStateUsage;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolumeState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<StorageVolumeStateUsage>,
}
