use bevy::prelude::*;

use super::SandboxBounds;

#[derive(Component)]
pub(crate) struct SandboxLight;

pub(crate) fn setup_lights(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

    // Key Light: brighter, casting the main shadows.
    commands.spawn((
        PointLight {
            intensity: 1_000_000_f32,
            shadows_enabled: true,
            ..default()
        },
        SandboxLight,
    ));

    // Fill Light: softer, helping to lift shadows.
    commands.spawn((
        PointLight {
            intensity: 600_000_f32,
            shadows_enabled: false,
            ..default()
        },
        SandboxLight,
    ));

    // Rim Light: adds a highlight to the edges.
    commands.spawn((
        PointLight {
            intensity: 400_000_f32,
            shadows_enabled: false,
            ..default()
        },
        SandboxLight,
    ));

    // Extra lights
    let num_extra_lights = 0;
    for _light_index in 0..num_extra_lights {
        commands.spawn((
            PointLight {
                intensity: 300_000_f32,
                shadows_enabled: true,
                ..default()
            },
            SandboxLight,
        ));
    }
}

pub(crate) fn update_lights(
    sandbox_bounds: Res<SandboxBounds>,
    mut light_query: Query<(&mut PointLight, &mut Transform), With<SandboxLight>>,
) {
    let SandboxBounds { center, extent } = sandbox_bounds.as_ref();

    let center = Vec3::from(*center);
    let radius = 0.6_f32 * extent.length() + 2_f32;
    let range = 2_f32 * radius;
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
