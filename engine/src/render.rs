use bevy::{prelude::*, utils::HashMap};
use villagekit_render::{Renderable, RenderableInstance, RenderableMaterial, RenderableMesh};

use crate::{assets::AssetStore, sandbox::Sandbox};

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct RenderableObject(pub Renderable);

pub fn spawn_renderable(renderable: Renderable, mut commands: Commands) {
    commands.spawn(RenderableObject(renderable));
}

pub(crate) fn process_renderables(
    mut commands: Commands,
    query: Query<(Entity, &RenderableObject), Added<RenderableObject>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut mesh_store: ResMut<AssetStore<RenderableMesh, Mesh>>,
    mut material_store: ResMut<AssetStore<RenderableMaterial, StandardMaterial>>,
    mut sandbox_query: Query<Entity, With<Sandbox>>,
) {
    let sandbox_entity = sandbox_query.single_mut();

    for (_entity, object) in query.iter() {
        let Renderable {
            meshes,
            materials,
            instances,
        } = &object.0;

        let mut meshes_by_id: HashMap<String, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<String, Handle<StandardMaterial>> = HashMap::new();

        for (id, mesh) in meshes {
            let handle = mesh_store.insert(mesh.clone(), mesh.clone().into(), &mut mesh_assets);
            meshes_by_id.insert(id.clone(), handle);
        }

        for (id, material) in materials {
            let handle = material_store.insert(
                material.clone(),
                material.clone().into(),
                &mut material_assets,
            );
            materials_by_id.insert(id.clone(), handle);
        }

        commands.entity(sandbox_entity).with_children(|parent| {
            for instance in instances {
                spawn_renderable_instance(
                    parent,
                    instance.clone(),
                    &meshes_by_id,
                    &materials_by_id,
                );
            }
        });
    }
}

fn spawn_renderable_instance(
    parent: &mut ChildBuilder,
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
        entity.insert(Into::<Transform>::into(transform));
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
