use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Lab {
    pub name: String,
    pub description: Option<String>,

    #[serde(default)]
    pub storage_pools: Vec<StoragePoolConfig>,

    #[serde(default)]
    pub networks: Vec<NetworkConfig>,

    #[serde(default)]
    pub profiles: Vec<ProfileConfig>,

    #[serde(default)]
    pub instances: Vec<InstanceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct StoragePoolConfig {
    pub name: String,
    pub driver: String,
    pub source: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub name: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub dns_domain: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub description: Option<String>,
    pub network: Option<String>,
    pub storage: Option<String>,
    #[serde(default)]
    pub config: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub image: String,
    pub network: Option<String>,
    #[serde(default)]
    pub profiles: Vec<String>,
    #[serde(default)]
    pub config: HashMap<String, String>,
    #[serde(default)]
    pub start: bool,
}
