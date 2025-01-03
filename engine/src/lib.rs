use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;

mod asset;
mod part;
mod product;
mod sandbox;

pub use crate::part::{spawn_part, Part, PartInstance, PartMaterial, PartMesh, PartSpec};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            bevy_editor_cam::DefaultEditorCamPlugins,
            InfiniteGridPlugin,
        ))
        .add_systems(Startup, crate::sandbox::setup_sandbox);
    }
}
