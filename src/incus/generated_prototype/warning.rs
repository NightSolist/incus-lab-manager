// Auto-generated. Do not edit.

use crate::incus::WarningPut;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Warning {
    #[serde(flatten)]
    pub warning_put: WarningPut,
    pub uuid: String,
    pub location: String,
    pub project: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub count: i64,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_message: String,
    pub severity: String,
    pub entity_url: String,
}
