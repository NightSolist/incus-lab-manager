// Auto-generated. Do not edit.

use crate::incus::NetworkForwardPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkForward {
    #[serde(flatten)]
    pub network_forward_put: NetworkForwardPut,
    pub listen_address: String,
    pub location: String,
}
