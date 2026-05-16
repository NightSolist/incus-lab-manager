// Auto-generated. Do not edit.

use serde::{Serialize, Deserialize};
use crate::incus::InstancePut;
use crate::incus::InstanceSource;
use crate::incus::InstanceType;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstancesPost {    #[serde(flatten)]    pub instance_put: InstancePut,    pub name: String,    pub source: InstanceSource,    pub instance_type: String,    #[serde(rename = "type")]    pub r#type: InstanceType,    pub start: bool,}