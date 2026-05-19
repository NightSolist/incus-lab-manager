// Auto-generated. Do not edit.

use crate::incus::ResourcesPCIVPD;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourcesPCIDevice {
    pub driver: String,
    pub driver_version: String,
    pub numa_node: u64,
    pub pci_address: String,
    pub vendor: String,
    pub vendor_id: String,
    pub product: String,
    pub product_id: String,
    pub iommu_group: u64,
    pub vpd: ResourcesPCIVPD,
}
