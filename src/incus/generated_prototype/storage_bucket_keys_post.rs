// Auto-generated. Do not edit.

use crate::incus::StorageBucketKeyPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBucketKeysPost {
    #[serde(flatten)]
    pub storage_bucket_key_put: StorageBucketKeyPut,
    pub name: String,
}
