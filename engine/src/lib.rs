use bevy::app::PostStartup;
use bevy::image::{ImageAddressMode, ImageSamplerDescriptor};
use bevy::prelude::{Mesh as BevyMesh, StandardMaterial as BevyStandardMaterial, *};
use bevy_infinite_grid::InfiniteGridPlugin;
use villagekit_product::Assembly;
use villagekit_product::ProductKind;
use villagekit_product::Stock;
use villagekit_render::{Material, Mesh};

mod assets;
mod product;
mod render;
mod sandbox;

pub(crate) use crate::assets::*;
pub(crate) use crate::product::*;
pub(crate) use crate::render::*;
pub(crate) use crate::sandbox::*;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(ImagePlugin {
                default_sampler: ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..Default::default()
                },
            }),
            MeshPickingPlugin,
            bevy_editor_cam::DefaultEditorCamPlugins,
            InfiniteGridPlugin,
        ))
        .insert_resource(AssetStore::<Mesh, BevyMesh>::new())
        .insert_resource(AssetStore::<Material, BevyStandardMaterial>::new())
        .add_systems(Startup, setup_sandbox)
        .add_systems(Update, process_products)
        .add_systems(Update, process_renderables);
    }
}

#[derive(Resource)]
struct ProductToSpawn(pub ProductKind);

pub fn setup_stock(stock: impl Stock + Send + Sync + 'static) {
    let product = ProductKind::Stock(Box::new(stock));
    App::new()
        .insert_resource(ProductToSpawn(product))
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, init_product)
        .run();
}

pub fn setup_assembly(assembly: impl Assembly + Send + Sync + 'static) {
    let product = ProductKind::Assembly(Box::new(assembly));
    App::new()
        .insert_resource(ProductToSpawn(product))
        .add_plugins(EnginePlugin)
        .add_systems(PostStartup, init_product)
        .run();
}

fn init_product(
    mut commands: Commands,
    sandbox_query: Query<Entity, With<Sandbox>>,
    product: Res<ProductToSpawn>,
) {
    let sandbox_entity = sandbox_query.single();
    spawn_product(sandbox_entity, product.0.clone().place(), &mut commands);
}
