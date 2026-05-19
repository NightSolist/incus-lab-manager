// Auto-generated. Do not edit.

use crate::incus::NetworkZonePut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkZonesPost {
    #[serde(flatten)]
    pub network_zone_put: NetworkZonePut,
    pub name: String,
}
