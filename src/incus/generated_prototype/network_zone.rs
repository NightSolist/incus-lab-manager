// Auto-generated. Do not edit.

use crate::incus::NetworkZonePut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkZone {
    #[serde(flatten)]
    pub network_zone_put: NetworkZonePut,
    pub name: String,
    pub used_by: Vec<String>,
    pub project: String,
}
