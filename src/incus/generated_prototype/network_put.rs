use serde::{Serialize, Deserialize};
use crate::incus::ConfigMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPut {
    pub config: ConfigMap,
    pub description: String,
}