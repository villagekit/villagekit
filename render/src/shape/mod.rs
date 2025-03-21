mod circle;
mod cuboid;

use bevy_math::{
    bounding::{Aabb2d, Aabb3d, BoundingVolume},
    Isometry2d, Isometry3d, Vec3A,
};
use bevy_render::mesh::Mesh;
use serde::{Deserialize, Serialize};

pub use self::circle::Circle;
pub use self::cuboid::Cuboid;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShapeId(String);

impl ShapeId {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

pub trait Bounded2d {
    fn bounds(&self, isometry: impl Into<Isometry2d>) -> Aabb2d;
}

pub trait Bounded3d {
    fn bounds(&self, isometry: impl Into<Isometry3d>) -> Aabb3d;
}

pub trait Meshable {
    fn mesh(&self) -> Mesh;
}

pub trait Shape2d: Bounded2d + Meshable {}
impl<T> Shape2d for T where T: Bounded2d + Meshable {}

pub trait Shape3d: Bounded3d + Meshable {}
impl<T> Shape3d for T where T: Bounded3d + Meshable {}

impl<T> Bounded3d for T
where
    T: Shape2d,
{
    fn bounds(&self, isometry: impl Into<Isometry3d>) -> Aabb3d {
        let isometry = isometry.into();
        let aabb2d = Bounded2d::bounds(self, Isometry2d::IDENTITY);
        let aabb3d = Aabb3d {
            min: Vec3A::new(aabb2d.min.x, 0., aabb2d.min.y),
            max: Vec3A::new(aabb2d.max.x, 0., aabb2d.max.y),
        };
        aabb3d.transformed_by(isometry.translation, isometry.rotation)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ShapeEnum {
    Circle(Circle),
    Cuboid(Cuboid),
}

impl ShapeEnum {
    pub fn bounds(&self, isometry: impl Into<Isometry3d>) -> Aabb3d {
        match self {
            ShapeEnum::Cuboid(shape) => Bounded3d::bounds(shape, isometry),
            ShapeEnum::Circle(shape) => Bounded3d::bounds(shape, isometry),
        }
    }
    pub fn mesh(&self) -> Mesh {
        match self {
            ShapeEnum::Cuboid(shape) => shape.mesh(),
            ShapeEnum::Circle(shape) => shape.mesh(),
        }
    }
}
