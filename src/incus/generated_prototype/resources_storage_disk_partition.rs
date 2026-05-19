// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesStorageDiskPartition {
    pub id: String,
    pub device: String,
    pub read_only: bool,
    pub size: u64,
    pub partition: u64,
}
