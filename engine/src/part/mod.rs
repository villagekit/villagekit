use bevy::{prelude::*, utils::HashMap};
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use crate::{assets::AssetStore, sandbox::Sandbox};

pub struct PartPlugin;

impl Plugin for PartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetStore::<PartMesh, Mesh>::new())
            .insert_resource(AssetStore::<PartMaterial, StandardMaterial>::new());
    }
}

#[derive(Component, Default)]
pub struct PartType;

#[derive(Component, Default)]
pub struct PartSpec;

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct PartInstance;

#[derive(Component, Default)]
pub struct PartRender {
    pub meshes: BTreeMap<String, PartMesh>,
    pub materials: BTreeMap<String, PartMaterial>,
    pub instances: Vec<PartSubInstance>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PartMesh {
    Cuboid {
        x_length: OrderedFloat<f32>,
        y_length: OrderedFloat<f32>,
        z_length: OrderedFloat<f32>,
    },
}

impl From<PartMesh> for Mesh {
    fn from(value: PartMesh) -> Self {
        value.mesh()
    }
}

impl PartMesh {
    fn mesh(&self) -> Mesh {
        match self {
            &PartMesh::Cuboid {
                x_length,
                y_length,
                z_length,
            } => Cuboid::new(*x_length, *y_length, *z_length).into(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PartColor {
    Hsla {
        hue: OrderedFloat<f32>,
        saturation: OrderedFloat<f32>,
        lightness: OrderedFloat<f32>,
        alpha: OrderedFloat<f32>,
    },
}

impl From<PartColor> for Color {
    fn from(value: PartColor) -> Self {
        match value {
            PartColor::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Color::hsla(*hue, *saturation, *lightness, *alpha),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PartMaterial {
    Color(PartColor),
}

impl PartMaterial {
    fn material(&self) -> StandardMaterial {
        match self {
            PartMaterial::Color(color) => StandardMaterial::from_color(color.clone()),
        }
    }
}

impl From<PartMaterial> for StandardMaterial {
    fn from(value: PartMaterial) -> Self {
        value.material()
    }
}

#[derive(Debug, Clone)]
pub struct PartSubInstance {
    pub mesh: Option<String>,
    pub material: Option<String>,
    pub transform: Option<Transform>,
    pub children: Option<Vec<PartSubInstance>>,
}

struct SpawnPartRender {
    render: PartRender,
}

impl Command for SpawnPartRender {
    fn apply(self, world: &mut World) {
        let mut meshes_by_id: HashMap<String, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

        world.resource_scope(|world, mut assets: Mut<Assets<Mesh>>| {
            world.resource_scope(|_world, mut store: Mut<AssetStore<PartMesh, Mesh>>| {
                for (id, mesh) in self.render.meshes {
                    let handle = store.insert(mesh, &mut assets);
                    meshes_by_id.insert(id, handle);
                }
            });
        });
        world.resource_scope(|world, mut assets: Mut<Assets<StandardMaterial>>| {
            world.resource_scope(
                |_world, mut store: Mut<AssetStore<PartMaterial, StandardMaterial>>| {
                    for (id, material) in self.render.materials {
                        let handle = store.insert(material, &mut assets);
                        materials_by_id.insert(id, handle);
                    }
                },
            );
        });

        let sandbox = world
            .query_filtered::<Entity, With<Sandbox>>()
            .get_single(world)
            .expect("Unable to get sandbox entity");

        world.entity_mut(sandbox).with_children(|parent| {
            parent.spawn(PartInstance).with_children(|parent| {
                for instance in self.render.instances {
                    spawn_part_sub_instance(parent, instance, &meshes_by_id, &materials_by_id);
                }
            });
        });
    }
}

pub fn spawn_part_render(render: PartRender, mut commands: Commands) {
    commands.queue(SpawnPartRender { render });
}

fn spawn_part_sub_instance(
    parent: &mut WorldChildBuilder,
    part_instance: PartSubInstance,
    meshes_by_id: &HashMap<String, Handle<Mesh>>,
    materials_by_id: &HashMap<String, Handle<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    if let Some(mesh_id) = part_instance.mesh {
        let mesh_handle = meshes_by_id
            .get(&mesh_id)
            .expect("Unable to get mesh by id");
        entity.insert(Mesh3d(mesh_handle.clone()));
    }

    if let Some(material_id) = part_instance.material {
        let material_handle = materials_by_id
            .get(&material_id)
            .expect("Unable to get material by id");
        entity.insert(MeshMaterial3d(material_handle.clone()));
    }

    if let Some(transform) = part_instance.transform {
        entity.insert(transform);
    }

    if let Some(children) = part_instance.children {
        entity.with_children(|parent| {
            for child_part_instance in children {
                spawn_part_sub_instance(parent, child_part_instance, meshes_by_id, materials_by_id);
            }
        });
    }
}
