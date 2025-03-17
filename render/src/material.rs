use std::collections::BTreeMap;

use bevy_asset::Handle;
use bevy_image::Image;
use bevy_pbr::StandardMaterial;
use bevy_render::alpha::AlphaMode as BevyAlphaMode;
use serde::{Deserialize, Serialize};
use villagekit_number::Number;

use crate::Color;

#[derive(Debug, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageId(String);

#[derive(Debug, Clone, Default)]
pub struct ImageStore(BTreeMap<ImageId, Handle<Image>>);

impl ImageStore {
    pub fn get(&self, id: &ImageId) -> Option<Handle<Image>> {
        self.0.get(id).cloned()
    }
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AlphaMode {
    #[default]
    Opaque,
    Mask(Number),
    Blend,
    Premultiplied,
    AlphaToCoverage,
    Add,
    Multiply,
}

impl From<AlphaMode> for BevyAlphaMode {
    fn from(value: AlphaMode) -> Self {
        match value {
            AlphaMode::Opaque => BevyAlphaMode::Opaque,
            AlphaMode::Mask(number) => BevyAlphaMode::Mask(number.into()),
            AlphaMode::Blend => BevyAlphaMode::Blend,
            AlphaMode::Premultiplied => BevyAlphaMode::Premultiplied,
            AlphaMode::AlphaToCoverage => BevyAlphaMode::AlphaToCoverage,
            AlphaMode::Add => BevyAlphaMode::Add,
            AlphaMode::Multiply => BevyAlphaMode::Multiply,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Material {
    base_color: Color,
    base_color_texture: Option<ImageId>,
    alpha_mode: AlphaMode,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            base_color_texture: None,
            alpha_mode: AlphaMode::default(),
        }
    }
}

impl Material {
    fn material(self, images: ImageStore) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color.into(),
            base_color_texture: self.base_color_texture.and_then(|i| images.get(&i)),
            alpha_mode: self.alpha_mode.into(),
            ..Default::default()
        }
    }
}
