// Auto-generated. Do not edit.

use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CertificateAddToken {
    pub client_name: String,
    pub fingerprint: String,
    pub addresses: Vec<String>,
    pub secret: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
