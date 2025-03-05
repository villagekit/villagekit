use bevy::prelude::*;
use bevy_editor_cam::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Sandbox;

pub(crate) fn setup_sandbox(mut commands: Commands) {
    /*
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    */

    // lights
    let num_lights = 6;
    for _light_index in 0..num_lights {
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                range: 100_f32,
                ..default()
            },
            Transform::from_xyz(
                rand::random_range(-20_f32..20_f32),
                rand::random_range(20_f32..30_f32),
                rand::random_range(-20_f32..20_f32),
            ),
        ));
    }

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
