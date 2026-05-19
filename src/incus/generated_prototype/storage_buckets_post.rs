// Auto-generated. Do not edit.

use crate::incus::StorageBucketPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBucketsPost {
    #[serde(flatten)]
    pub storage_bucket_put: StorageBucketPut,
    pub name: String,
}
