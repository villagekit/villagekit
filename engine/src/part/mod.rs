use bevy::prelude::*;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use crate::{asset::AssetStore, sandbox::Sandbox};

pub struct PartPlugin;

impl Plugin for PartPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetStore::<PartMesh, Mesh, _>::new(|key: &PartMesh| {
            key.mesh()
        }))
        .insert_resource(AssetStore::<PartMaterial, StandardMaterial, _>::new(
            |key: &PartMaterial| key.material(),
        ));
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
    pub instances: Vec<PartInstance>,
}

#[derive(Component, Default)]
pub struct PartRenderHandles {
    pub mesh_handles: BTreeMap<String, Handle<Mesh>>,
    pub material_handles: BTreeMap<String, Handle<StandardMaterial>>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PartMesh {
    Cuboid {
        x_length: OrderedFloat<f32>,
        y_length: OrderedFloat<f32>,
        z_length: OrderedFloat<f32>,
    },
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

#[derive(Debug, Clone)]
pub struct PartSubInstance {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
    pub transform: Option<Transform>,
    pub children: Option<Vec<PartSubInstance>>,
}

struct SpawnPartRender {
    render: PartRender,
}

impl Command for SpawnPartRender {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut assets: Mut<Assets<Mesh>>| {
            world.resource_scope(|_world, mut store: Mut<AssetStore<PartMesh, Mesh, _>>| {
                for (id, mesh) in self.render.meshes {
                    store.get_or_create(mesh, &mut assets);
                }
            });
        });
        world.resource_scope(|world, mut assets: Mut<Assets<StandardMaterial>>| {
            world.resource_scope(
                |_world, mut store: Mut<AssetStore<PartMaterial, StandardMaterial, _>>| {
                    for (id, material) in self.render.materials {
                        store.get_or_create(material, &mut assets);
                    }
                },
            );
        });

        let sandbox = world
            .query_filtered::<Entity, With<Sandbox>>()
            .get_single(world)
            .expect("Unable to get sandbox entity");

        world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
            world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
                world.entity_mut(sandbox).with_children(|parent| {
                    parent.spawn(Part).with_children(|parent| {
                        for part_instance in self.part_spec.instances {
                            spawn_part_instance(
                                parent,
                                part_instance,
                                meshes.reborrow(),
                                materials.reborrow(),
                            );
                        }
                    });
                });
            });
        });
    }
}

pub fn spawn_part(part_spec: PartSpec, mut commands: Commands) {
    commands.queue(SpawnPart { part_spec });
}

fn spawn_part_instance(
    parent: &mut WorldChildBuilder,
    part_instance: PartInstance,
    mut meshes: Mut<Assets<Mesh>>,
    mut materials: Mut<Assets<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    if let Some(mesh_id) = part_instance.mesh {
        let mesh_handle = Handle::weak_from_u128(mesh_id.as_u128());
        entity.insert(Mesh3d(mesh_handle));
    }

    if let Some(material_id) = part_instance.material {
        let material_handle = Handle::<StandardMaterial>::weak_from_u128(material_id.as_u128());
        entity.insert(MeshMaterial3d(material_handle));
    }

    if let Some(transform) = part_instance.transform {
        entity.insert(transform);
    }

    if let Some(children) = part_instance.children {
        entity.with_children(|parent| {
            for child_part_instance in children {
                spawn_part_instance(
                    parent,
                    child_part_instance,
                    meshes.reborrow(),
                    materials.reborrow(),
                );
            }
        });
    }
}
