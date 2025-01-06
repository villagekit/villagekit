use bevy::{prelude::*, utils::HashMap};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{assets::AssetStore, sandbox::Sandbox};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub meshes: BTreeMap<String, RenderableMesh>,
    pub materials: BTreeMap<String, RenderableMaterial>,
    pub instances: Vec<RenderableInstance>,
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct RenderableObject;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RenderableMesh {
    Cuboid {
        x_length: OrderedFloat<f32>,
        y_length: OrderedFloat<f32>,
        z_length: OrderedFloat<f32>,
    },
}

impl From<RenderableMesh> for Mesh {
    fn from(value: RenderableMesh) -> Self {
        value.mesh()
    }
}

impl RenderableMesh {
    fn mesh(&self) -> Mesh {
        match self {
            &RenderableMesh::Cuboid {
                x_length,
                y_length,
                z_length,
            } => Cuboid::new(*x_length, *y_length, *z_length).into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RenderableColor {
    Hsla {
        hue: OrderedFloat<f32>,
        saturation: OrderedFloat<f32>,
        lightness: OrderedFloat<f32>,
        alpha: OrderedFloat<f32>,
    },
}

impl From<RenderableColor> for Color {
    fn from(value: RenderableColor) -> Self {
        match value {
            RenderableColor::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Color::hsla(*hue, *saturation, *lightness, *alpha),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum RenderableMaterial {
    Color(RenderableColor),
}

impl RenderableMaterial {
    fn material(&self) -> StandardMaterial {
        match self {
            RenderableMaterial::Color(color) => StandardMaterial::from_color(color.clone()),
        }
    }
}

impl From<RenderableMaterial> for StandardMaterial {
    fn from(value: RenderableMaterial) -> Self {
        value.material()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderableInstance {
    pub mesh: Option<String>,
    pub material: Option<String>,
    pub transform: Option<Transform>,
    pub children: Option<Vec<RenderableInstance>>,
}

struct SpawnRenderable {
    renderable: Renderable,
}

impl Command for SpawnRenderable {
    fn apply(self, world: &mut World) {
        let mut meshes_by_id: HashMap<String, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

        world.resource_scope(|world, mut assets: Mut<Assets<Mesh>>| {
            world.resource_scope(|_world, mut store: Mut<AssetStore<RenderableMesh, Mesh>>| {
                for (id, mesh) in self.renderable.meshes {
                    let handle = store.insert(mesh.clone(), mesh.into(), &mut assets);
                    meshes_by_id.insert(id, handle);
                }
            });
        });
        world.resource_scope(|world, mut assets: Mut<Assets<StandardMaterial>>| {
            world.resource_scope(
                |_world, mut store: Mut<AssetStore<RenderableMaterial, StandardMaterial>>| {
                    for (id, material) in self.renderable.materials {
                        let handle = store.insert(material.clone(), material.into(), &mut assets);
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
            parent.spawn(RenderableObject).with_children(|parent| {
                for instance in self.renderable.instances {
                    spawn_renderable_instance(parent, instance, &meshes_by_id, &materials_by_id);
                }
            });
        });
    }
}

pub fn spawn_renderable(renderable: Renderable, mut commands: Commands) {
    commands.queue(SpawnRenderable { renderable });
}

fn spawn_renderable_instance(
    parent: &mut WorldChildBuilder,
    instance: RenderableInstance,
    meshes_by_id: &HashMap<String, Handle<Mesh>>,
    materials_by_id: &HashMap<String, Handle<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    if let Some(mesh_id) = instance.mesh {
        let mesh_handle = meshes_by_id
            .get(&mesh_id)
            .expect("Unable to get mesh by id");
        entity.insert(Mesh3d(mesh_handle.clone()));
    }

    if let Some(material_id) = instance.material {
        let material_handle = materials_by_id
            .get(&material_id)
            .expect("Unable to get material by id");
        entity.insert(MeshMaterial3d(material_handle.clone()));
    }

    if let Some(transform) = instance.transform {
        entity.insert(transform);
    }

    if let Some(children) = instance.children {
        entity.with_children(|parent| {
            for child_part_instance in children {
                spawn_renderable_instance(
                    parent,
                    child_part_instance,
                    meshes_by_id,
                    materials_by_id,
                );
            }
        });
    }
}
