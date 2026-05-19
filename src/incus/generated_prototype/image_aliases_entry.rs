// Auto-generated. Do not edit.

use crate::incus::ImageAliasesEntryPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageAliasesEntry {
    #[serde(flatten)]
    pub image_aliases_entry_put: ImageAliasesEntryPut,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}
