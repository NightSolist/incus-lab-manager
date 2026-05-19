// Auto-generated. Do not edit.

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstanceStateNetworkCounters {
    pub bytes_received: i64,
    pub bytes_sent: i64,
    pub packets_received: i64,
    pub packets_sent: i64,
    pub errors_received: i64,
    pub errors_sent: i64,
    pub packets_dropped_outbound: i64,
    pub packets_dropped_inbound: i64,
}
