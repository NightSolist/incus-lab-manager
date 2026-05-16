// Auto-generated. Do not edit.

use crate::incus::NetworkPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworksPost {
    #[serde(flatten)]
    pub network_put: NetworkPut,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
