use bevy::prelude::*;

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
