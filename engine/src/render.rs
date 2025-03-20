use bevy::{math::bounding::Aabb3d, prelude::*, utils::HashMap};
use std::{
    borrow::BorrowMut,
    sync::{Arc, Weak},
};
use villagekit_render::{
    ImageId, Instance as RenderableInstance, Material as RenderableMaterial,
    MaterialId as RenderableMaterialId, Renderable, Shape, ShapeEnum as RenderableShape,
    ShapeId as RenderableShapeId,
};

use crate::AssetStore;

#[derive(Component)]
#[require(Transform, Visibility)]
pub(crate) struct RenderableObject(pub Renderable);

#[derive(Resource)]
pub(crate) struct ShapesById(pub HashMap<RenderableShapeId, Weak<RenderableShape>>);

impl ShapesById {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // Insert by taking a strong reference (Arc<T>) and storing a Weak<T>.
    pub fn insert(&mut self, key: RenderableShapeId, value: Arc<RenderableShape>) {
        let weak = Arc::downgrade(&value);
        self.0.insert(key, weak);
    }

    // Attempt to get a strong reference (Arc<T>) back by upgrading the Weak<T>.
    // If upgrade() fails, it means the value is already dropped.
    pub fn get(&self, key: &RenderableShapeId) -> Option<Arc<RenderableShape>> {
        self.0.get(key).and_then(|weak_ref| weak_ref.upgrade())
    }

    pub fn retain_alive(&mut self) {
        self.0.retain(|_, weak_ref| weak_ref.upgrade().is_some());
    }
}

#[derive(Component)]
#[require(GlobalTransform)]
pub(crate) struct ShapeObject(pub RenderableShapeId);

pub fn spawn_renderable(parent: Entity, renderable: Renderable, commands: &mut Commands) {
    commands.entity(parent).with_children(|p| {
        p.spawn(RenderableObject(renderable));
    });
}

pub(crate) fn process_renderables(
    mut commands: Commands,
    query: Query<(Entity, &RenderableObject), Added<RenderableObject>>,
    mut shapes_by_id: ResMut<ShapesById>,
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

        let mut meshes_by_id: HashMap<RenderableShapeId, Handle<Mesh>> = HashMap::new();
        let mut materials_by_id: HashMap<RenderableMaterialId, Handle<StandardMaterial>> =
            HashMap::new();

        for (id, shape) in shapes {
            shapes_by_id.insert(id.clone(), shape.clone().into());
            let mesh_handle = mesh_store.insert(shape.clone(), shape.mesh(), &mut mesh_assets);
            meshes_by_id.insert(id.clone(), mesh_handle);
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
    meshes_by_id: &HashMap<RenderableShapeId, Handle<Mesh>>,
    materials_by_id: &HashMap<RenderableMaterialId, Handle<StandardMaterial>>,
) {
    let mesh_handle = meshes_by_id
        .get(&instance.shape)
        .expect("Unable to get mesh by id");
    let material_handle = materials_by_id
        .get(&instance.material)
        .expect("Unable to get material by id");

    parent
        .spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            Transform::from(instance.transform),
            ShapeObject(instance.shape),
        ))
        .with_children(|parent| {
            for child_part_instance in instance.children {
                spawn_renderable_instance(
                    parent,
                    child_part_instance,
                    meshes_by_id,
                    materials_by_id,
                );
            }
        });
}
