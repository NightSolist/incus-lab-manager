// Auto-generated. Do not edit.

use crate::incus::ResourcesStoragePoolInodes;
use crate::incus::ResourcesStoragePoolSpace;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesStoragePool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<ResourcesStoragePoolSpace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inodes: Option<ResourcesStoragePoolInodes>,
}
