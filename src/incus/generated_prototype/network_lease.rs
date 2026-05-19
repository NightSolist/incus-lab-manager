// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkLease {
    pub hostname: String,
    pub hwaddr: String,
    pub address: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub location: String,
}
