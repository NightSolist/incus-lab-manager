// Auto-generated. Do not edit.

use crate::incus::EventLifecycleRequestor;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventLifecycle {
    pub action: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor: Option<EventLifecycleRequestor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
