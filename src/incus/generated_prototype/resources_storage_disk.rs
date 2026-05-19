// Auto-generated. Do not edit.

use crate::incus::ResourcesStorageDiskPartition;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesStorageDisk {
    pub id: String,
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub read_only: bool,
    pub size: u64,
    pub removable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wwn: Option<String>,
    pub numa_node: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_path: Option<String>,
    pub block_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    pub rpm: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    pub device_id: String,
    pub partitions: Vec<ResourcesStorageDiskPartition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pci_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usb_address: Option<String>,
}
