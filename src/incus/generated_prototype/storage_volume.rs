// Auto-generated. Do not edit.

use crate::incus::StorageVolumePut;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageVolume {
    #[serde(flatten)]
    pub storage_volume_put: StorageVolumePut,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub used_by: Vec<String>,
    pub location: String,
    pub content_type: String,
    pub project: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
