// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkAddress {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
