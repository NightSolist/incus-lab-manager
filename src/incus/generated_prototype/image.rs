// Auto-generated. Do not edit.

use crate::incus::ImageAlias;
use crate::incus::ImagePut;
use crate::incus::ImageSource;
use chrono;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(flatten)]
    pub image_put: ImagePut,
    pub aliases: Vec<ImageAlias>,
    pub architecture: String,
    pub cached: bool,
    pub filename: String,
    pub fingerprint: String,
    pub size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_source: Option<ImageSource>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: chrono::DateTime<chrono::Utc>,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub project: String,
}
