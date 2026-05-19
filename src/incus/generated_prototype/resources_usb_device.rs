// Auto-generated. Do not edit.

use crate::incus::ResourcesUSBDeviceInterface;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesUSBDevice {
    pub bus_address: u64,
    pub device_address: u64,
    pub serial: String,
    pub interfaces: Vec<ResourcesUSBDeviceInterface>,
    pub vendor: String,
    pub vendor_id: String,
    pub product: String,
    pub product_id: String,
    pub speed: f64,
}
