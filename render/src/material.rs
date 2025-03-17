use bevy_asset::Handle;
use bevy_image::Image;
use bevy_pbr::StandardMaterial;
use bevy_render::alpha::AlphaMode as BevyAlphaMode;
use serde::{Deserialize, Serialize};
use villagekit_number::Number;

use crate::{Color, ImageId};

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
    pub base_color: Color,
    pub base_color_texture: Option<ImageId>,
    pub alpha_mode: AlphaMode,
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
    pub fn to_bevy(self, get_image: impl Fn(ImageId) -> Handle<Image>) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color.into(),
            base_color_texture: self.base_color_texture.map(get_image),
            alpha_mode: self.alpha_mode.into(),
            ..Default::default()
        }
    }
}
