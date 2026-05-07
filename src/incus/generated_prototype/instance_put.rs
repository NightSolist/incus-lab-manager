use serde::{Serialize, Deserialize};
use crate::incus::ConfigMap;
use crate::incus::DevicesMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstancePut {
    pub architecture: String,
    pub config: ConfigMap,
    pub devices: DevicesMap,
    pub ephemeral: bool,
    pub profiles: Vec<String>,
    pub restore: Option<String>,
    pub disk_only: Option<bool>,
    pub stateful: Option<bool>,
    pub description: String,
}