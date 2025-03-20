mod cuboid;

use bevy_math::{bounding::Aabb3d, Isometry3d};
use bevy_render::mesh::Mesh;
use serde::{Deserialize, Serialize};

pub use self::cuboid::Cuboid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapeId(String);

impl ShapeId {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

pub trait Bounded {
    fn bounds(&self, isometry: impl Into<Isometry3d>) -> Aabb3d;
}

pub trait Meshable {
    fn mesh(&self) -> Mesh;
}

pub trait Shape: Bounded + Meshable {}

impl<T> Shape for T where T: Bounded + Meshable {}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ShapeEnum {
    Cuboid(Cuboid),
}

impl ShapeEnum {
    pub fn bounds(&self, isometry: impl Into<Isometry3d>) -> Aabb3d {
        match self {
            ShapeEnum::Cuboid(cuboid) => cuboid.bounds(isometry),
        }
    }
    pub fn mesh(&self) -> Mesh {
        match self {
            ShapeEnum::Cuboid(cuboid) => cuboid.mesh(),
        }
    }
}
