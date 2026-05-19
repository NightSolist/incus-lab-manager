// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupTarget {
    pub protocol: String,
    pub url: String,
    pub bucket_name: String,
    pub path: String,
    pub access_key: String,
    pub secret_key: String,
}
