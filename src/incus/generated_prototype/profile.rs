// Auto-generated. Do not edit.

use crate::incus::ProfilePut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    #[serde(flatten)]
    pub profile_put: ProfilePut,
    pub name: String,
    pub used_by: Vec<String>,
    pub project: String,
}
