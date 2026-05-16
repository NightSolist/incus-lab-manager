// Auto-generated. Do not edit.

use crate::incus::ProfilePut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfilesPost {
    #[serde(flatten)]
    pub profile_put: ProfilePut,
    pub name: String,
}
