// Auto-generated. Do not edit.

use crate::incus::StorageBucketPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBucket {
    #[serde(flatten)]
    pub storage_bucket_put: StorageBucketPut,
    pub name: String,
    pub s3_url: String,
    pub location: String,
    pub project: String,
}
