// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceStateMemory {
    pub usage: i64,
    pub usage_peak: i64,
    pub total: i64,
    pub swap_usage: i64,
    pub swap_usage_peak: i64,
}
