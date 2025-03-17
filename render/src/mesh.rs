use bevy_math::prelude::Cuboid;
use bevy_render::mesh::{Mesh as BevyMesh, VertexAttributeValues};
use serde::{Deserialize, Serialize};
use villagekit_unit::{Dimension, Length};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Mesh {
    Cuboid {
        x_length: Length,
        y_length: Length,
        z_length: Length,
    },
}

impl From<Mesh> for BevyMesh {
    fn from(value: Mesh) -> Self {
        value.mesh()
    }
}

impl Mesh {
    fn mesh(&self) -> BevyMesh {
        match self {
            &Mesh::Cuboid {
                x_length,
                y_length,
                z_length,
            } => {
                let x_length = x_length.canonical().into();
                let y_length = y_length.canonical().into();
                let z_length = z_length.canonical().into();
                let mut mesh: BevyMesh = Cuboid::new(x_length, y_length, z_length).into();

                // Map UVs from (0, 0) -> (1, 1) to world units
                if let Some(VertexAttributeValues::Float32x2(ref mut uvs)) =
                    mesh.attribute_mut(BevyMesh::ATTRIBUTE_UV_0)
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
        }
    }
}
