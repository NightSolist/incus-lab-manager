// Auto-generated. Do not edit.

use crate::incus::ImageSource;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImagesPostSource {
    #[serde(flatten)]
    pub image_source: ImageSource,
    pub mode: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub url: String,
    pub name: String,
    pub fingerprint: String,
    pub secret: String,
    pub project: String,
}
