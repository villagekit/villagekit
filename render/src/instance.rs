use serde::{Deserialize, Serialize};

use crate::Transform;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshId(String);

impl MeshId {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialId(String);

impl MaterialId {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub mesh: MeshId,
    pub material: MaterialId,
    #[serde(default)]
    pub transform: Transform,
    #[serde(default)]
    pub children: Vec<Instance>,
}
