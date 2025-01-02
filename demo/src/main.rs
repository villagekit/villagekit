use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_editor_cam::prelude::EditorCam;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGridSettings};
use uuid::Uuid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            bevy_editor_cam::DefaultEditorCamPlugins,
            InfiniteGridPlugin,
        ))
        .add_systems(Startup, setup_sandbox)
        .add_systems(Startup, setup_model)
        .run();
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
struct Sandbox;

fn setup_sandbox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 4.0, 12.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        EditorCam::default(),
    ));

    // grid
    commands.spawn((InfiniteGridBundle {
        settings: InfiniteGridSettings {
            fadeout_distance: 1000.,
            ..Default::default()
        },
        ..Default::default()
    },));

    // sandbox
    commands.spawn((
        Sandbox,
        // z-up to y-up matrix
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}

fn setup_model(mut commands: Commands) {
    let cube_id = Uuid::new_v4();
    let part_meshes = BTreeMap::from([(
        cube_id,
        PartMesh::Cuboid {
            x_length: 1.0,
            y_length: 1.0,
            z_length: 10.0,
        },
    )]);
    let white_id = Uuid::new_v4();
    let part_materials = BTreeMap::from([(
        white_id,
        PartMaterial::Color {
            color: Color::WHITE,
        },
    )]);
    let part_entities = vec![PartEntity {
        mesh: Some(cube_id),
        material: Some(white_id),
        transform: Some(Transform::from_xyz(0.0, 0.5, 0.0)),
        children: None,
    }];
    let part_spec = PartSpec {
        meshes: part_meshes,
        materials: part_materials,
        entities: part_entities,
    };

    spawn_part(part_spec, commands.reborrow());
}

#[derive(Component, Default)]
#[require(Transform, Visibility)]
struct Part;

#[derive(Clone, Default)]
struct PartSpec {
    meshes: BTreeMap<Uuid, PartMesh>,
    materials: BTreeMap<Uuid, PartMaterial>,
    entities: Vec<PartEntity>,
}

#[derive(Debug, Clone)]
enum PartMesh {
    Cuboid {
        x_length: f32,
        y_length: f32,
        z_length: f32,
    },
}

#[derive(Debug, Clone)]
enum PartMaterial {
    Color { color: Color },
}

#[derive(Debug, Clone)]
struct PartEntity {
    pub mesh: Option<Uuid>,
    pub material: Option<Uuid>,
    pub transform: Option<Transform>,
    pub children: Option<Vec<PartEntity>>,
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
                        for entity in self.part_spec.entities {
                            spawn_part_entity(
                                parent,
                                entity,
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

fn spawn_part(part_spec: PartSpec, mut commands: Commands) {
    commands.queue(SpawnPart { part_spec });
}

fn spawn_part_entity(
    parent: &mut WorldChildBuilder,
    part_entity: PartEntity,
    mut meshes: Mut<Assets<Mesh>>,
    mut materials: Mut<Assets<StandardMaterial>>,
) {
    let mut entity = parent.spawn_empty();

    if let Some(mesh_id) = part_entity.mesh {
        let mesh_handle = Handle::weak_from_u128(mesh_id.as_u128());
        entity.insert(Mesh3d(mesh_handle));
    }

    if let Some(material_id) = part_entity.material {
        let material_handle = Handle::<StandardMaterial>::weak_from_u128(material_id.as_u128());
        entity.insert(MeshMaterial3d(material_handle));
    }

    if let Some(transform) = part_entity.transform {
        entity.insert(transform);
    }

    if let Some(children) = part_entity.children {
        entity.with_children(|parent| {
            for child_part_entity in children {
                spawn_part_entity(
                    parent,
                    child_part_entity,
                    meshes.reborrow(),
                    materials.reborrow(),
                );
            }
        });
    }
}
