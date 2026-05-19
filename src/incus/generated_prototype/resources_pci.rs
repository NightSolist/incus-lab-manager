// Auto-generated. Do not edit.

use crate::incus::ResourcesPCIDevice;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesPCI {
    pub devices: Vec<ResourcesPCIDevice>,
    pub total: u64,
}
