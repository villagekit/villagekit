use bevy_math::prelude::Cuboid;
use bevy_render::mesh::Mesh as BevyMesh;
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
            } => Cuboid::new(
                x_length.canonical().into(),
                y_length.canonical().into(),
                z_length.canonical().into(),
            )
            .into(),
        }
    }
}
