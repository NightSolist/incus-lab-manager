// Auto-generated. Do not edit.

use crate::incus::BackupTarget;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolumeBackupsPost {
    pub name: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub volume_only: bool,
    pub optimized_storage: bool,
    pub compression_algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<BackupTarget>,
}
