use bevy::{
    math::{
        bounding::{Aabb3d, BoundingVolume},
        Vec3A,
    },
    prelude::*,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridSettings};

mod camera;
mod lights;

pub(crate) use camera::{setup_camera, update_camera};
pub(crate) use lights::{setup_lights, update_lights};

use crate::{ShapeObject, ShapesById};

#[derive(Component, Default)]
#[require(Transform, Visibility)]
pub struct Sandbox;

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

#[derive(Resource)]
pub struct SandboxBounds {
    center: Vec3A,
    extent: Vec3A,
}

impl Default for SandboxBounds {
    fn default() -> Self {
        Self {
            center: Vec3A::new(0., 0., 0.),
            extent: Vec3A::new(1., 1., 1.),
        }
    }
}

pub(crate) fn update_sandbox_bounds(
    sandbox_query: Query<Entity, With<Sandbox>>,
    children_query: Query<&Children>,
    bounded_query: Query<(&ShapeObject, &GlobalTransform)>,
    shapes_by_id: Res<ShapesById>,
    mut current_sandbox_bounds: ResMut<SandboxBounds>,
) {
    let sandbox_entity = match sandbox_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    let mut sandbox_bounds = Aabb3d {
        min: Vec3A::splat(f32::INFINITY),
        max: Vec3A::splat(f32::NEG_INFINITY),
    };

    for entity in children_query.iter_descendants(sandbox_entity) {
        if let Ok((entity_shape_id, entity_global_transform)) = bounded_query.get(entity) {
            let isometry = entity_global_transform.to_isometry();
            let entity_shape = shapes_by_id
                .get(&entity_shape_id.0)
                .expect("Failed to get shape");
            let entity_bounds = entity_shape.bounds(isometry);
            sandbox_bounds = sandbox_bounds.merge(&entity_bounds);
        }
    }

    let center = sandbox_bounds.center();
    let extent = sandbox_bounds.max - sandbox_bounds.min;

    if !extent.is_finite() {
        return;
    };

    if current_sandbox_bounds.center != center || current_sandbox_bounds.extent != extent {
        let b = current_sandbox_bounds.as_mut();
        b.center = center;
        b.extent = extent;
    }
}
