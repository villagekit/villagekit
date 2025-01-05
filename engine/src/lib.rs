use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;
use part::PartPlugin;

mod assets;
mod entry;
mod part;
mod product;
mod sandbox;
mod workspace;

pub use crate::part::{
    spawn_part_render, PartColor, PartInstance, PartMaterial, PartMesh, PartRender, PartSpec,
    PartSubInstance,
};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            bevy_editor_cam::DefaultEditorCamPlugins,
            InfiniteGridPlugin,
        ))
        .add_plugins(PartPlugin)
        .add_systems(Startup, crate::sandbox::setup_sandbox);
    }
}
