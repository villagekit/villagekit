use assets::AssetStore;
use bevy::prelude::{
    App, DefaultPlugins, Mesh, MeshPickingPlugin, Plugin, StandardMaterial, Startup, Update,
};
use bevy_infinite_grid::InfiniteGridPlugin;
pub use villagekit_math::*;
pub use villagekit_number::*;
pub use villagekit_product::*;
pub use villagekit_render::*;
pub use villagekit_unit::*;

mod assets;
mod product;
mod render;
mod sandbox;

pub(crate) use crate::assets::*;
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
        .add_systems(Startup, setup_sandbox)
        .add_systems(Update, process_products)
        .add_systems(Update, process_renderables);
    }
}
