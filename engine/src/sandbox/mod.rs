use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

mod camera;
mod lights;

pub(crate) use camera::{setup_camera, update_camera};
pub(crate) use lights::{setup_lights, update_lights};

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Sandbox;

#[derive(Resource)]
pub struct SandboxBounds {
    center: Vec3,
    extent: Vec3,
}

impl Default for SandboxBounds {
    fn default() -> Self {
        Self {
            center: Vec3::new(0., 0., 0.),
            extent: Vec3::new(1., 1., 1.),
        }
    }
}

pub(crate) fn setup_sandbox(mut commands: Commands) {
    // grid
    commands.spawn((InfiniteGridBundle {
        settings: InfiniteGridSettings {
            fadeout_distance: 1000.,
            scale: 100.,
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

pub(crate) fn update_sandbox_bounds(
    sandbox_query: Query<Entity, With<Sandbox>>,
    children_query: Query<&Children>,
    transform_query: Query<&GlobalTransform>,
    mut sandbox_bounds: ResMut<SandboxBounds>,
) {
    let sandbox_entity = match sandbox_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for entity in children_query.iter_descendants(sandbox_entity) {
        if let Ok(global_transform) = transform_query.get(entity) {
            let pos = global_transform.translation();
            min = min.min(pos);
            max = max.max(pos);
        }
    }

    let center = 0.5_f32 * (min + max);
    let extent = max - min;

    if !extent.is_finite() {
        return;
    };

    if sandbox_bounds.center != center || sandbox_bounds.extent != extent {
        sandbox_bounds.center = center;
        sandbox_bounds.extent = extent;
    }
}
