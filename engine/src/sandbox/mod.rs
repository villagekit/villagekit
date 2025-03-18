use bevy::prelude::*;
use bevy_editor_cam::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

mod lights;

pub(crate) use lights::{setup_lights, update_lights};

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Sandbox;

pub(crate) fn setup_sandbox(mut commands: Commands) {
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
