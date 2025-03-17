use bevy_asset::Handle;
use bevy_image::Image;
use bevy_pbr::{
    OpaqueRendererMethod as BevyOpaqueRenderMethod,
    ParallaxMappingMethod as BevyParallaxMappingMethod, StandardMaterial,
    UvChannel as BevyUvChannel,
};
use bevy_render::{alpha::AlphaMode as BevyAlphaMode, render_resource::Face as BevyFace};
use serde::{Deserialize, Serialize};
use villagekit_number::Number;
use villagekit_unit::num;

use crate::{Color, ImageId};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Material {
    pub base_color: Color,
    pub base_color_channel: UvChannel,
    pub base_color_texture: Option<ImageId>,

    pub emissive: Color,
    pub emissive_exposure_weight: Number,
    pub emissive_channel: UvChannel,
    pub emissive_texture: Option<ImageId>,

    pub perceptual_roughness: Number,
    pub metallic: Number,
    pub metallic_roughness_channel: UvChannel,
    pub metallic_roughness_texture: Option<ImageId>,

    pub reflectance: Number,

    pub diffuse_transmission: Number,
    pub specular_transmission: Number,
    pub thickness: Number,
    pub ior: Number,

    pub attenuation_distance: Number,
    pub attenuation_color: Color,

    pub normal_map_channel: UvChannel,
    pub normal_map_texture: Option<ImageId>,
    pub flip_normal_map_y: bool,

    pub occlusion_channel: UvChannel,
    pub occlusion_texture: Option<ImageId>,

    pub clearcoat: Number,
    pub clearcoat_perceptual_roughness: Number,

    pub anisotropy_strength: Number,
    pub anisotropy_rotation: Number,

    pub double_sided: bool,
    pub cull_mode: Option<Face>,

    pub unlit: bool,
    pub fog_enabled: bool,

    pub alpha_mode: AlphaMode,

    pub depth_bias: Number,
    pub depth_map: Option<ImageId>,
    pub parallax_depth_scale: Number,
    pub parallax_mapping_method: ParallaxMappingMethod,
    pub max_parallax_layer_count: Number,

    pub lightmap_exposure: Number,
    pub opaque_render_method: OpaqueRendererMethod,
    pub deferred_lighting_pass_id: u8,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            base_color_channel: UvChannel::default(),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_exposure_weight: num!(0.0),
            emissive_channel: UvChannel::default(),
            emissive_texture: None,
            perceptual_roughness: num!(0.5),
            metallic: num!(0.0),
            metallic_roughness_channel: UvChannel::default(),
            metallic_roughness_texture: None,
            reflectance: num!(0.5),
            diffuse_transmission: num!(0.0),
            specular_transmission: num!(0.0),
            thickness: num!(0.0),
            ior: num!(1.5),
            attenuation_distance: Number::INFINITY,
            attenuation_color: Color::WHITE,
            normal_map_channel: UvChannel::default(),
            normal_map_texture: None,
            flip_normal_map_y: false,
            occlusion_channel: UvChannel::default(),
            occlusion_texture: None,
            clearcoat: num!(0.0),
            clearcoat_perceptual_roughness: num!(0.5),
            anisotropy_strength: num!(0.0),
            anisotropy_rotation: num!(0.0),
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            fog_enabled: true,
            alpha_mode: AlphaMode::default(),
            depth_bias: num!(0.0),
            depth_map: None,
            parallax_depth_scale: num!(0.1),
            parallax_mapping_method: ParallaxMappingMethod::default(),
            max_parallax_layer_count: num!(16.0),
            lightmap_exposure: num!(1.0),
            opaque_render_method: OpaqueRendererMethod::Forward,
            deferred_lighting_pass_id: 0,
        }
    }
}

impl Material {
    /// Converts this custom Material into Bevy's StandardMaterial.
    /// The `get_image` function is used to convert an ImageId to a Handle<Image>.
    pub fn to_bevy(self, get_image: impl Fn(ImageId) -> Handle<Image>) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color.into(),
            base_color_channel: self.base_color_channel.into(),
            base_color_texture: self.base_color_texture.map(&get_image),
            emissive: self.emissive.into(),
            emissive_exposure_weight: self.emissive_exposure_weight.into(),
            emissive_channel: self.emissive_channel.into(),
            emissive_texture: self.emissive_texture.map(&get_image),
            perceptual_roughness: self.perceptual_roughness.into(),
            metallic: self.metallic.into(),
            metallic_roughness_channel: self.metallic_roughness_channel.into(),
            metallic_roughness_texture: self.metallic_roughness_texture.map(&get_image),
            reflectance: self.reflectance.into(),
            diffuse_transmission: self.diffuse_transmission.into(),
            specular_transmission: self.specular_transmission.into(),
            thickness: self.thickness.into(),
            ior: self.ior.into(),
            attenuation_distance: self.attenuation_distance.into(),
            attenuation_color: self.attenuation_color.into(),
            normal_map_channel: self.normal_map_channel.into(),
            normal_map_texture: self.normal_map_texture.map(&get_image),
            flip_normal_map_y: self.flip_normal_map_y,
            occlusion_channel: self.occlusion_channel.into(),
            occlusion_texture: self.occlusion_texture.map(&get_image),
            clearcoat: self.clearcoat.into(),
            clearcoat_perceptual_roughness: self.clearcoat_perceptual_roughness.into(),
            anisotropy_strength: self.anisotropy_strength.into(),
            anisotropy_rotation: self.anisotropy_rotation.into(),
            double_sided: self.double_sided,
            cull_mode: self.cull_mode.map(Into::into),
            unlit: self.unlit,
            fog_enabled: self.fog_enabled,
            alpha_mode: self.alpha_mode.into(),
            depth_bias: self.depth_bias.into(),
            depth_map: self.depth_map.map(&get_image),
            parallax_depth_scale: self.parallax_depth_scale.into(),
            parallax_mapping_method: self.parallax_mapping_method.into(),
            max_parallax_layer_count: self.max_parallax_layer_count.into(),
            lightmap_exposure: self.lightmap_exposure.into(),
            opaque_render_method: self.opaque_render_method.into(),
            deferred_lighting_pass_id: self.deferred_lighting_pass_id,
            ..Default::default()
        }
    }
}

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    Front,
    Back,
}

impl From<Face> for BevyFace {
    fn from(value: Face) -> Self {
        match value {
            Face::Front => BevyFace::Front,
            Face::Back => BevyFace::Back,
        }
    }
}

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum OpaqueRendererMethod {
    #[default]
    Forward,
    Deferred,
    Auto,
}

impl From<OpaqueRendererMethod> for BevyOpaqueRenderMethod {
    fn from(value: OpaqueRendererMethod) -> Self {
        match value {
            OpaqueRendererMethod::Forward => BevyOpaqueRenderMethod::Forward,
            OpaqueRendererMethod::Deferred => BevyOpaqueRenderMethod::Deferred,
            OpaqueRendererMethod::Auto => BevyOpaqueRenderMethod::Auto,
        }
    }
}

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum ParallaxMappingMethod {
    #[default]
    Occlusion,
    Relief {
        max_steps: u32,
    },
}

impl From<ParallaxMappingMethod> for BevyParallaxMappingMethod {
    fn from(value: ParallaxMappingMethod) -> Self {
        match value {
            ParallaxMappingMethod::Occlusion => BevyParallaxMappingMethod::Occlusion,
            ParallaxMappingMethod::Relief { max_steps } => {
                BevyParallaxMappingMethod::Relief { max_steps }
            }
        }
    }
}

#[derive(
    Debug, Copy, Clone, Default, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum UvChannel {
    #[default]
    Uv0,
    Uv1,
}

impl From<UvChannel> for BevyUvChannel {
    fn from(value: UvChannel) -> Self {
        match value {
            UvChannel::Uv0 => BevyUvChannel::Uv0,
            UvChannel::Uv1 => BevyUvChannel::Uv1,
        }
    }
}
