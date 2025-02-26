use bevy_color::Color;
use bevy_math::prelude::Cuboid;
use bevy_pbr::StandardMaterial;
use bevy_render::mesh::Mesh;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub meshes: BTreeMap<String, RenderableMesh>,
    pub materials: BTreeMap<String, RenderableMaterial>,
    pub instances: Vec<RenderableInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RenderableMesh {
    Cuboid {
        x_length: OrderedFloat<f32>,
        y_length: OrderedFloat<f32>,
        z_length: OrderedFloat<f32>,
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
            } => Cuboid::new(*x_length, *y_length, *z_length).into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum RenderableColor {
    Hsla {
        hue: OrderedFloat<f32>,
        saturation: OrderedFloat<f32>,
        lightness: OrderedFloat<f32>,
        alpha: OrderedFloat<f32>,
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
            } => Color::hsla(*hue, *saturation, *lightness, *alpha),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
