// Auto-generated. Do not edit.

use serde::{Serialize, Deserialize};
use crate::incus::ProfilePut;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilesPost {    #[serde(flatten)]    pub profile_put: ProfilePut,    pub name: String,}