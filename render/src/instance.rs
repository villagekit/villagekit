use serde::{Deserialize, Serialize};

use crate::{MaterialId, ShapeId, Transform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub shape: ShapeId,
    pub material: MaterialId,
    #[serde(default)]
    pub transform: Transform,
    #[serde(default)]
    pub children: Vec<Instance>,
}
