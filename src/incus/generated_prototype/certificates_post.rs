// Auto-generated. Do not edit.

use crate::incus::CertificatePut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CertificatesPost {
    #[serde(flatten)]
    pub certificate_put: CertificatePut,
    pub trust_token: String,
    pub token: bool,
}
