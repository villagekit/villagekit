use bevy::prelude::*;
use bevy_editor_cam::prelude::EditorCam;

use super::SandboxBounds;

#[derive(Component)]
pub(crate) struct SandboxCamera;

pub(crate) fn setup_camera(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-1.2, 1.2, 2.).looking_at(Vec3::ZERO, Vec3::Y),
        EditorCam::default(),
        SandboxCamera,
    ));
}

pub(crate) fn update_camera(
    sandbox_bounds: Res<SandboxBounds>,
    mut camera_query: Query<(&mut Camera3d, &mut Transform), With<SandboxCamera>>,
) {
    let SandboxBounds { center, extent } = sandbox_bounds.as_ref();
    let center = Vec3::from(*center);

    let radius = 0.5 * extent.length();

    let (mut _camera, mut camera_transform) = camera_query.single_mut();
    *camera_transform =
        Transform::from_translation(center + Vec3::new(-1.2 * radius, 1.2 * radius, 2. * radius))
            .looking_at(center, Vec3::Y);
}
