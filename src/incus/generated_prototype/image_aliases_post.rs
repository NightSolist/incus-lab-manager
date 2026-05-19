// Auto-generated. Do not edit.

use crate::incus::ImageAliasesEntry;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageAliasesPost {
    #[serde(flatten)]
    pub image_aliases_entry: ImageAliasesEntry,
}
