// Auto-generated. Do not edit.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json;
use chrono;
use crate::incus::StatusCode;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Operation {    pub id: String,    pub class: String,    pub description: String,    pub created_at: chrono::DateTime<chrono::Utc>,    pub updated_at: chrono::DateTime<chrono::Utc>,    pub status: String,    pub status_code: StatusCode,    pub resources: HashMap<String, Vec<String>>,    pub metadata: HashMap<String, serde_json::Value>,    pub may_cancel: bool,    pub err: String,    pub location: String,}