// Auto-generated. Do not edit.

use crate::incus::ServerEnvironment;
use crate::incus::ServerUntrusted;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Server {
    #[serde(flatten)]
    pub server_untrusted: ServerUntrusted,
    pub auth_user_name: String,
    pub auth_user_method: String,
    pub environment: ServerEnvironment,
}
