use bevy::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::sandbox::Sandbox;

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Part;

#[derive(Clone, Default)]
pub struct PartSpec {
    pub meshes: BTreeMap<Uuid, PartMesh>,
    pub materials: BTreeMap<Uuid, PartMaterial>,
    pub instances: Vec<PartInstance>,
}

#[derive(Debug, Clone)]
pub enum PartMesh {
    Cuboid {
        x_length: f32,
        y_length: f32,
        z_length: f32,
    },
}

#[derive(Debug, Clone)]
pub enum PartMaterial {
    Color { color: Color },
}

#[derive(Debug, Clone)]
pub struct PartInstance {
    pub mesh: Option<Uuid>,
    pub material: Option<Uuid>,
    pub transform: Option<Transform>,
    pub children: Option<Vec<PartInstance>>,
}

struct SpawnPart {
    part_spec: PartSpec,
}

impl Command for SpawnPart {
    fn apply(self, world: &mut World) {
        world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
            for (id, part_mesh) in self.part_spec.meshes {
                let mesh: Mesh = match part_mesh {
                    PartMesh::Cuboid {
                        x_length,
                        y_length,
                        z_length,
                    } => Cuboid::new(x_length, y_length, z_length).into(),
                };
                let mesh_handle = Handle::weak_from_u128(id.as_u128());
                meshes.insert(&mesh_handle, mesh);
            }
        });
        world.resource_scope(|_world, mut materials: Mut<Assets<StandardMaterial>>| {
            for (id, part_material) in self.part_spec.materials {
                let material = match part_material {
                    PartMaterial::Color { color } => StandardMaterial::from_color(color),
                };
                let material_handle = Handle::weak_from_u128(id.as_u128());
                materials.insert(&material_handle, material);
            }
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
