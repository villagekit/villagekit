use bevy_math::{
    bounding::{Aabb2d, Bounded2d as BevyBounded2d},
    prelude::Circle as BevyCircle,
    Isometry2d,
};
use bevy_render::mesh::{Mesh, Meshable as BevyMeshable};
use serde::{Deserialize, Serialize};
use villagekit_unit::{Dimension, Length};

use super::{Bounded2d, Meshable, ShapeEnum};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Circle {
    pub radius: Length,
}

impl From<Circle> for ShapeEnum {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}

impl From<&Circle> for BevyCircle {
    fn from(value: &Circle) -> Self {
        Self::new(value.radius.canonical().into())
    }
}

impl Bounded2d for Circle {
    fn bounds(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
        BevyCircle::from(self).aabb_2d(isometry)
    }
}

impl Meshable for Circle {
    fn mesh(&self) -> Mesh {
        let _radius: f32 = self.radius.canonical().into();

        let mesh: Mesh = BevyCircle::from(self).mesh().into();

        // TODO: Map UVs from (0, 0) -> (1, 1) to world units

        mesh
    }
}
