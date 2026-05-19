// Auto-generated. Do not edit.

use crate::incus::ProjectStateResource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectState {
    pub resources: HashMap<String, ProjectStateResource>,
}
