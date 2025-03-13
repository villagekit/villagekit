use bevy_color::Color;
use bevy_math::prelude::Cuboid;
use bevy_pbr::StandardMaterial;
use bevy_render::mesh::Mesh;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use villagekit_number::Number;
use villagekit_unit::{Dimension, Length};

use crate::Transform;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Renderable {
    pub meshes: BTreeMap<String, RenderableMesh>,
    pub materials: BTreeMap<String, RenderableMaterial>,
    pub instances: Vec<RenderableInstance>,
}

impl Renderable {
    pub fn insert_mesh(mut self, key: String, mesh: RenderableMesh) -> Self {
        self.meshes.insert(key, mesh);
        self
    }
    pub fn insert_material(mut self, key: String, material: RenderableMaterial) -> Self {
        self.materials.insert(key, material);
        self
    }
    pub fn insert_instance(mut self, instance: RenderableInstance) -> Self {
        // TODO check that meshes and materials exist, for children too.
        self.instances.push(instance);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RenderableMesh {
    Cuboid {
        x_length: Length,
        y_length: Length,
        z_length: Length,
    },
}

impl From<RenderableMesh> for Mesh {
    fn from(value: RenderableMesh) -> Self {
        value.mesh()
    }
}

impl RenderableMesh {
    fn mesh(&self) -> Mesh {
        match self {
            &RenderableMesh::Cuboid {
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

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RenderableColor {
    Hsla {
        hue: Number,
        saturation: Number,
        lightness: Number,
        alpha: Number,
    },
}

impl From<RenderableColor> for Color {
    fn from(value: RenderableColor) -> Self {
        match value {
            RenderableColor::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Color::hsla(
                hue.into(),
                saturation.into(),
                lightness.into(),
                alpha.into(),
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RenderableMaterial {
    Color { color: RenderableColor },
}

impl RenderableMaterial {
    fn material(&self) -> StandardMaterial {
        match self {
            RenderableMaterial::Color { color } => StandardMaterial::from_color(color.clone()),
        }
    }
}

impl From<RenderableMaterial> for StandardMaterial {
    fn from(value: RenderableMaterial) -> Self {
        value.material()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderableInstance {
    #[serde(default)]
    pub mesh: Option<String>,
    #[serde(default)]
    pub material: Option<String>,
    #[serde(default)]
    pub transform: Option<Transform>,
    #[serde(default)]
    pub children: Option<Vec<RenderableInstance>>,
}
