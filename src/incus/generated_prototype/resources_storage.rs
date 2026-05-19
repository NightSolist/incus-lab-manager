// Auto-generated. Do not edit.

use crate::incus::ResourcesStorageDisk;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesStorage {
    pub disks: Vec<ResourcesStorageDisk>,
    pub total: u64,
}
