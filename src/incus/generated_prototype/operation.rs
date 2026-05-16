// Auto-generated. Do not edit.

use crate::incus::StatusCode;
use chrono;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Operation {
    pub id: String,
    pub class: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub status_code: StatusCode,
    pub resources: HashMap<String, Vec<String>>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub may_cancel: bool,
    pub err: String,
    pub location: String,
}
