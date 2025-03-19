use serde::{Deserialize, Serialize};

use crate::{MaterialId, Shape3dId, Transform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub mesh: Shape3dId,
    pub material: MaterialId,
    #[serde(default)]
    pub transform: Transform,
    #[serde(default)]
    pub children: Vec<Instance>,
}
