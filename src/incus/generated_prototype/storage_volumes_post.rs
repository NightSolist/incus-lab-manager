// Auto-generated. Do not edit.

use crate::incus::StorageVolumePut;
use crate::incus::StorageVolumeSource;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolumesPost {
    #[serde(flatten)]
    pub storage_volume_put: StorageVolumePut,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub source: StorageVolumeSource,
    pub content_type: String,
}
