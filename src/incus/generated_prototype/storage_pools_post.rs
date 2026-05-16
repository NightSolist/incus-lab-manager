// Auto-generated. Do not edit.

use crate::incus::StoragePoolPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePoolsPost {
    #[serde(flatten)]
    pub storage_pool_put: StoragePoolPut,
    pub name: String,
    pub driver: String,
}
