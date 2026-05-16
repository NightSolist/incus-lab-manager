// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceStatePut {
    pub action: String,
    pub timeout: i64,
    pub force: bool,
    pub stateful: bool,
}
