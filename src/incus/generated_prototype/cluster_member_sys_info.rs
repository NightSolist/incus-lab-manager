// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterMemberSysInfo {
    pub uptime: i64,
    pub load_averages: Vec<f64>,
    pub total_ram: u64,
    pub free_ram: u64,
    pub shared_ram: u64,
    #[serde(rename = "buffered_ram")]
    pub buffer_ram: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub processes: u16,
}
