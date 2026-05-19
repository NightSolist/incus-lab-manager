// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesUSBDeviceInterface {
    pub class: String,
    pub class_id: u64,
    pub driver: String,
    pub driver_version: String,
    pub number: u64,
    #[serde(rename = "subclass")]
    pub sub_class: String,
    #[serde(rename = "subclass_id")]
    pub sub_class_id: u64,
}
