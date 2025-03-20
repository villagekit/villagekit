use bevy::{
    pbr::light_consts::lux::{AMBIENT_DAYLIGHT, OVERCAST_DAY},
    prelude::*,
};

use super::SandboxBounds;

#[derive(Component)]
#[require(DirectionalLight)]
pub(crate) struct SandboxLight;

pub(crate) fn setup_lights(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

    // Key Light: brightest with shadows enabled.
    let up = Dir3::Y;
    let key_dir = Dir3::X.slerp(Dir3::Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
    commands.spawn((
        SandboxLight,
        DirectionalLight {
            illuminance: 5_000.,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(key_dir, up),
    ));

    // Fill Light: softer light to lift shadows.
    let fill_dir = Dir3::X.slerp(Dir3::NEG_Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
    commands.spawn((
        SandboxLight,
        DirectionalLight {
            illuminance: 1_000.,
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_to(fill_dir, up),
    ));

    // Rim Light: provides edge highlights.
    let rim_dir = Dir3::NEG_X.slerp(Dir3::NEG_Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
    commands.spawn((
        SandboxLight,
        DirectionalLight {
            illuminance: 500.,
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_to(rim_dir, up),
    ));

    // Extra background lights (if needed)
    let num_extra_lights = 0;
    for _ in 0..num_extra_lights {
        commands.spawn((
            DirectionalLight {
                illuminance: 300.0,
                shadows_enabled: true,
                ..default()
            },
            SandboxLight,
        ));
    }
}

/*
pub(crate) fn update_lights(
    sandbox_bounds: Res<SandboxBounds>,
    mut light_query: Query<(&mut DirectionalLight, &mut Transform), With<SandboxLight>>,
) {
    let SandboxBounds { center, extent } = sandbox_bounds.as_ref();
    let center = Vec3::from(*center);
    // Determine a distance offset based on the scene's bounding box.
    let radius = 0.6 * extent.length() + 2.0;
    let num_lights = light_query.iter().len();

    for (i, (_light, mut transform)) in light_query.iter_mut().enumerate() {
        // Position the lights around the center for 3‑point (and extra) lighting.
        let light_pos = match i {
            0 => center + Vec3::new(radius, radius, radius), // Key light: above & right.
            1 => center + Vec3::new(-radius, radius, radius), // Fill light: above & left.
            2 => center + Vec3::new(0.0, radius, -radius),   // Rim light: above & behind.
            _ => {
                // Extra lights arranged in a circle around the scene.
                let angle = i as f32 * std::f32::consts::TAU / (num_lights as f32);
                center + Vec3::new(angle.cos() * radius, radius, angle.sin() * radius)
            }
        };

        // For directional lights the translation isn’t used for falloff,
        // but we set it so we can derive a useful direction.
        // The transform is rotated so that the light “looks at” the center.
        *transform = Transform::from_translation(light_pos).looking_at(center, Vec3::Y);
    }
}
*/
