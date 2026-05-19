// Auto-generated. Do not edit.

use crate::incus::ResourcesStoragePool;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePoolState {
    #[serde(flatten)]
    pub resources_storage_pool: ResourcesStoragePool,
}
