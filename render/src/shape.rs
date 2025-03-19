use bevy_math::{
    bounding::{Aabb3d, Bounded3d},
    prelude::Cuboid as BevyCuboid,
    Isometry3d,
};
use bevy_render::mesh::{Mesh, Meshable, VertexAttributeValues};
use serde::{Deserialize, Serialize};
use villagekit_unit::{Dimension, Length};

use crate::Transform;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Shape3dId(String);

impl Shape3dId {
    pub fn new(key: &str) -> Self {
        Self(key.into())
    }
}

pub trait Shape3d {
    fn mesh(&self) -> Mesh;
    fn bounds(&self, transform: Transform) -> Aabb3d;
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
struct Cuboid {
    x_length: Length,
    y_length: Length,
    z_length: Length,
}

impl Cuboid {
    fn to_bevy_shape(&self) -> BevyCuboid {
        BevyCuboid::new(
            self.x_length.canonical().into(),
            self.y_length.canonical().into(),
            self.z_length.canonical().into(),
        )
    }
}

impl Shape3d for Cuboid {
    fn mesh(&self) -> Mesh {
        let x_length = self.x_length.canonical().into();
        let y_length = self.y_length.canonical().into();
        let z_length = self.z_length.canonical().into();

        let mut mesh: Mesh = self.to_bevy_shape().mesh().into();

        // Map UVs from (0, 0) -> (1, 1) to world units
        if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0)
        {
            for (i, uv) in uvs.iter_mut().enumerate() {
                *uv = match i {
                    // Front face: lies in the X–Y plane.
                    // Use x_length for U and y_length for V.
                    0 => [0.0, 0.0],
                    1 => [x_length, 0.0],
                    2 => [x_length, y_length],
                    3 => [0.0, y_length],
                    // Back face: also spans X–Y.
                    // Note: the vertex order is reversed compared to the front.
                    4 => [x_length, 0.0],
                    5 => [0.0, 0.0],
                    6 => [0.0, y_length],
                    7 => [x_length, y_length],
                    // Right face: lies in the Y–Z plane.
                    // Use z_length for U and y_length for V.
                    8 => [0.0, 0.0],
                    9 => [z_length, 0.0],
                    10 => [z_length, y_length],
                    11 => [0.0, y_length],
                    // Left face: also in the Y–Z plane.
                    // Vertex order is reversed compared to the right face.
                    12 => [z_length, 0.0],
                    13 => [0.0, 0.0],
                    14 => [0.0, y_length],
                    15 => [z_length, y_length],
                    // Top face: lies in the X–Z plane.
                    // Use x_length for U and z_length for V.
                    16 => [x_length, 0.0],
                    17 => [0.0, 0.0],
                    18 => [0.0, z_length],
                    19 => [x_length, z_length],
                    // Bottom face: also in the X–Z plane.
                    // Here we flip the V mapping relative to the top.
                    20 => [0.0, 0.0],
                    21 => [x_length, 0.0],
                    22 => [x_length, z_length],
                    23 => [0.0, z_length],
                    _ => unreachable!(),
                }
            }
        }

        mesh
    }

    fn bounds(&self, transform: Transform) -> Aabb3d {
        let isometry: Isometry3d = transform.into();
        self.to_bevy_shape().aabb_3d(isometry)
    }
}
