use assets::AssetStore;
use bevy::prelude::*;
use bevy_infinite_grid::InfiniteGridPlugin;
pub use villagekit_render::{Renderable, RenderableInstance, RenderableMaterial, RenderableMesh};

mod assets;
mod product;
mod render;
mod sandbox;

pub use crate::assets::*;
pub use crate::product::*;
pub use crate::render::*;
pub use crate::sandbox::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            bevy_editor_cam::DefaultEditorCamPlugins,
            InfiniteGridPlugin,
        ))
        .insert_resource(AssetStore::<RenderableMesh, Mesh>::new())
        .insert_resource(AssetStore::<RenderableMaterial, StandardMaterial>::new())
        .add_systems(Startup, crate::sandbox::setup_sandbox)
        .add_systems(Update, crate::render::process_renderables);
    }
}
