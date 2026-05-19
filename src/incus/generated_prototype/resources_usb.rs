// Auto-generated. Do not edit.

use crate::incus::ResourcesUSBDevice;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesUSB {
    pub devices: Vec<ResourcesUSBDevice>,
    pub total: u64,
}
