// Auto-generated. Do not edit.

use crate::incus::NetworkACLPost;
use crate::incus::NetworkACLPut;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkACLsPost {
    #[serde(flatten)]
    pub network_acl_post: NetworkACLPost,
    #[serde(flatten)]
    pub network_acl_put: NetworkACLPut,
}
