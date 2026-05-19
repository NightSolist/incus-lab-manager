// Auto-generated. Do not edit.

use crate::incus::ServerPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerUntrusted {
    #[serde(flatten)]
    pub server_put: ServerPut,
    pub api_extensions: Vec<String>,
    pub api_status: String,
    pub api_version: String,
    pub auth: String,
    pub public: bool,
    pub auth_methods: Vec<String>,
}
