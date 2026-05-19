// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesStoragePoolSpace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<u64>,
    pub total: u64,
}
