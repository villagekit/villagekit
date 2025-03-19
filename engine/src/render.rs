use bevy::{math::bounding::Aabb3d, prelude::*, utils::HashMap};
use villagekit_render::{
    ImageId, Instance as RenderableInstance, Material as RenderableMaterial,
    MaterialId as RenderableMaterialId, Renderable, Shape, ShapeEnum as RenderableShape,
    ShapeId as RenderableShapeId,
};

use crate::AssetStore;

#[derive(Component)]
#[require(Transform, Visibility)]
pub(crate) struct RenderableObject(pub Renderable);

#[derive(Component)]
pub(crate) struct RenderableBounds(Aabb3d);

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
    mut mesh_store: ResMut<AssetStore<RenderableShape, Mesh>>,
    mut material_store: ResMut<AssetStore<RenderableMaterial, StandardMaterial>>,
    server: Res<AssetServer>,
) {
    for (entity, object) in query.iter() {
        let Renderable {
            shapes,
            materials,
            instances,
        } = &object.0;

        let mut shapes_by_id: HashMap<RenderableShapeId, RenderableShape> = HashMap::new();
        let mut meshes_by_id: HashMap<RenderableShapeId, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<RenderableMaterialId, Handle<StandardMaterial>> =
            HashMap::new();

        for (id, shape) in shapes {
            shapes_by_id.insert(id.clone(), shape.clone());
            let handle = mesh_store.insert(shape.clone(), shape.mesh(), &mut mesh_assets);
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
                    &shapes_by_id,
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
    shapes_by_id: &HashMap<RenderableShapeId, RenderableShape>,
    meshes_by_id: &HashMap<RenderableShapeId, Handle<Mesh>>,
    materials_by_id: &HashMap<RenderableMaterialId, Handle<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    let mesh_handle = meshes_by_id
        .get(&instance.shape)
        .expect("Unable to get mesh by id");
    entity.insert(Mesh3d(mesh_handle.clone()));

    let material_handle = materials_by_id
        .get(&instance.material)
        .expect("Unable to get material by id");
    entity.insert(MeshMaterial3d(material_handle.clone()));

    entity.insert(Into::<Transform>::into(instance.transform));

    let shape = shapes_by_id
        .get(&instance.shape)
        .expect("Unable to get shape by id");
    let bounds = shape.bounds(instance.transform);
    entity.insert(RenderableBounds(bounds));

    entity.with_children(|parent| {
        for child_part_instance in instance.children {
            spawn_renderable_instance(
                parent,
                child_part_instance,
                shapes_by_id,
                meshes_by_id,
                materials_by_id,
            );
        }
    });
}
