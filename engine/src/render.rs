use bevy::{prelude::*, utils::HashMap};
use villagekit_render::{
    ImageId, Instance as RenderableInstance, Material as RenderableMaterial,
    MaterialId as RenderableMaterialId, Mesh as RenderableMesh, MeshId as RenderableMeshId,
    Renderable,
};

use crate::AssetStore;

#[derive(Component)]
#[require(Transform, Visibility)]
pub(crate) struct RenderableObject(pub Renderable);

pub fn spawn_renderable(parent: Entity, renderable: Renderable, commands: &mut Commands) {
    commands.entity(parent).with_children(|p| {
        p.spawn(RenderableObject(renderable));
    });
}

pub(crate) fn process_renderables(
    mut commands: Commands,
    query: Query<(Entity, &RenderableObject), Added<RenderableObject>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut mesh_store: ResMut<AssetStore<RenderableMesh, Mesh>>,
    mut material_store: ResMut<AssetStore<RenderableMaterial, StandardMaterial>>,
    server: Res<AssetServer>,
) {
    for (entity, object) in query.iter() {
        let Renderable {
            meshes,
            materials,
            instances,
        } = &object.0;

        let mut meshes_by_id: HashMap<RenderableMeshId, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<RenderableMaterialId, Handle<StandardMaterial>> =
            HashMap::new();

        for (id, mesh) in meshes {
            let handle = mesh_store.insert(mesh.clone(), mesh.clone().into(), &mut mesh_assets);
            meshes_by_id.insert(id.clone(), handle);
        }

        let get_image = |image_id: ImageId| server.load(image_id.as_ref());

        for (id, material) in materials {
            let handle = material_store.insert(
                material.clone(),
                material.clone().to_bevy(get_image),
                &mut material_assets,
            );
            materials_by_id.insert(id.clone(), handle);
        }

        commands.entity(entity).with_children(|parent| {
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
    meshes_by_id: &HashMap<RenderableMeshId, Handle<Mesh>>,
    materials_by_id: &HashMap<RenderableMaterialId, Handle<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    let mesh_handle = meshes_by_id
        .get(&instance.mesh)
        .expect("Unable to get mesh by id");
    entity.insert(Mesh3d(mesh_handle.clone()));

    let material_handle = materials_by_id
        .get(&instance.material)
        .expect("Unable to get material by id");
    entity.insert(MeshMaterial3d(material_handle.clone()));

    entity.insert(Into::<Transform>::into(instance.transform));

    entity.with_children(|parent| {
        for child_part_instance in instance.children {
            spawn_renderable_instance(parent, child_part_instance, meshes_by_id, materials_by_id);
        }
    });
}
