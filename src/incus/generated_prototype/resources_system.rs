// Auto-generated. Do not edit.

use crate::incus::ResourcesSystemChassis;
use crate::incus::ResourcesSystemFirmware;
use crate::incus::ResourcesSystemMotherboard;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesSystem {
    pub uuid: String,
    pub vendor: String,
    pub product: String,
    pub family: String,
    pub version: String,
    pub sku: String,
    pub serial: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware: Option<ResourcesSystemFirmware>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis: Option<ResourcesSystemChassis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motherboard: Option<ResourcesSystemMotherboard>,
}
