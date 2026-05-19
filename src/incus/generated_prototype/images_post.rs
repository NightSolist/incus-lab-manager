// Auto-generated. Do not edit.

use crate::incus::ImageAlias;
use crate::incus::ImagePut;
use crate::incus::ImagesPostSource;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImagesPost {
    #[serde(flatten)]
    pub image_put: ImagePut,
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImagesPostSource>,
    pub compression_algorithm: String,
    pub format: String,
    pub aliases: Vec<ImageAlias>,
}
