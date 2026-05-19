// Auto-generated. Do not edit.

use crate::incus::config_map::{deserialize_config_map, deserialize_option_config_map};
use crate::incus::ConfigMap;
use crate::incus::StorageVolumeSnapshotPut;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolumeSnapshot {
    #[serde(flatten)]
    pub storage_volume_snapshot_put: StorageVolumeSnapshotPut,
    pub name: String,
    #[serde(deserialize_with = "deserialize_config_map")]
    pub config: ConfigMap,
    pub content_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
