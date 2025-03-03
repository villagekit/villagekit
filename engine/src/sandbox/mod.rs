use bevy::prelude::*;
use bevy_editor_cam::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Sandbox;

pub(crate) fn setup_sandbox(
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
