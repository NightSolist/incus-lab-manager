use serde::{Serialize, Deserialize};
use crate::incus::ConfigMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPut {
    pub config: ConfigMap,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}