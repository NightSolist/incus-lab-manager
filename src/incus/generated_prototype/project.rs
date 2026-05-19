// Auto-generated. Do not edit.

use crate::incus::ProjectPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(flatten)]
    pub project_put: ProjectPut,
    pub name: String,
    pub used_by: Vec<String>,
}
