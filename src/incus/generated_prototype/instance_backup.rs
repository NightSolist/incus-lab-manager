// Auto-generated. Do not edit.

use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceBackup {
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub instance_only: bool,
    pub optimized_storage: bool,
}
