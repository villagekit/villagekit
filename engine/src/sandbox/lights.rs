use bevy::prelude::*;

use super::Sandbox;

#[derive(Component)]
pub(crate) struct StudioLight;

pub(crate) fn setup_lights(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

    // Key Light: brighter, casting the main shadows.
    commands.spawn((
        PointLight {
            intensity: 1_500_000_f32,
            shadows_enabled: true,
            ..default()
        },
        StudioLight,
    ));

    // Fill Light: softer, helping to lift shadows.
    commands.spawn((
        PointLight {
            intensity: 1_000_000_f32,
            shadows_enabled: false,
            ..default()
        },
        StudioLight,
    ));

    // Rim Light: adds a highlight to the edges.
    commands.spawn((
        PointLight {
            intensity: 500_000_f32,
            shadows_enabled: false,
            ..default()
        },
        StudioLight,
    ));

    // Extra lights
    let num_extra_lights = 6;
    for _light_index in 0..num_extra_lights {
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                ..default()
            },
            StudioLight,
        ));
    }
}

/// Each frame, this system calculates the axis–aligned bounding box for all entities
/// that are descendants of the Sandbox entity, and then repositions the studio lights around that bounding box.
pub(crate) fn update_lights(
    sandbox_query: Query<Entity, With<Sandbox>>,
    children_query: Query<&Children>,
    transform_query: Query<&GlobalTransform>,
    mut light_query: Query<(&mut PointLight, &mut Transform), With<StudioLight>>,
) {
    // For simplicity, assume there is a single Sandbox entity.
    let sandbox_entity = match sandbox_query.get_single() {
        Ok(entity) => entity,
        Err(_) => return,
    };

    // Collect all descendant entities of the sandbox.
    let descendant_entities = collect_descendants(sandbox_entity, &children_query);

    if descendant_entities.is_empty() {
        return;
    }

    // Compute the bounding box of all descendant entities.
    let mut min = Vec3::splat(f32::INFINITY);
    let mut max = Vec3::splat(f32::NEG_INFINITY);
    for entity in descendant_entities {
        if let Ok(global_transform) = transform_query.get(entity) {
            let pos = global_transform.translation();
            min = min.min(pos);
            max = max.max(pos);
        }
    }

    let center = 0.5_f32 * (min + max);
    let extent = max - min;
    let radius = 0.5_f32 * extent.length() + 5.0;
    let range = 3_f32 * extent.length();
    let num_extra_lights = light_query.iter().len();

    // Update the positions of the studio lights around the bounding box.
    for (i, (mut light, mut light_transform)) in light_query.iter_mut().enumerate() {
        match i {
            0 => {
                // Key light: placed above and to the right.
                light.range = range;
                *light_transform =
                    Transform::from_translation(center + Vec3::new(radius, radius, radius));
            }
            1 => {
                light.range = range;
                // Fill light: placed above and to the left.
                *light_transform =
                    Transform::from_translation(center + Vec3::new(-radius, radius, radius));
            }
            2 => {
                light.range = range;
                // Rim light: placed behind the scene.
                *light_transform =
                    Transform::from_translation(center + Vec3::new(0.0, radius, -radius));
            }
            _ => {
                // Additional lights arranged in a circle.
                let angle = i as f32 * std::f32::consts::TAU / (num_extra_lights as f32);
                let offset = Vec3::new(angle.cos() * radius, radius, angle.sin() * radius);
                light.range = range;
                *light_transform = Transform::from_translation(center + offset);
            }
        }
    }
}

/// Recursively collects all descendant entities of a given parent.
fn collect_descendants(entity: Entity, children_query: &Query<&Children>) -> Vec<Entity> {
    let mut descendants = Vec::new();
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            descendants.push(child);
            descendants.extend(collect_descendants(child, children_query));
        }
    }
    descendants
}
