use bevy::prelude::*;

#[derive(Component)]
#[require(DirectionalLight)]
pub(crate) struct SandboxLight;

pub(crate) fn setup_lights(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

    // Key Light: brightest with shadows enabled.
    let up = Dir3::Y;
    let key_dir = Dir3::X.slerp(Dir3::NEG_Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
    commands.spawn((
        SandboxLight,
        DirectionalLight {
            illuminance: 3_000.,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(key_dir, up),
    ));

    // Fill Light: softer light to lift shadows.
    let fill_dir = Dir3::NEG_X.slerp(Dir3::NEG_Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
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
    let rim_dir = Dir3::X.slerp(Dir3::Z, 0.5).slerp(Dir3::NEG_Y, 0.5);
    commands.spawn((
        SandboxLight,
        DirectionalLight {
            illuminance: 500.,
            shadows_enabled: false,
            ..default()
        },
        Transform::default().looking_to(rim_dir, up),
    ));
}
