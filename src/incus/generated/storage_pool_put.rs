use serde::{Serialize, Deserialize};
use crate::incus::ConfigMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePoolPut {
    config: ConfigMap,
    description: String,
}