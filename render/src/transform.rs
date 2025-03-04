use bevy_math::Vec3;
use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Quaternion, Transform3, Vector3};
use villagekit_number::{num, Number};
use villagekit_unit::Length;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Transform {
    translation: Vector3<Length>,
    rotation: Quaternion,
}

impl Transform {
    pub fn set_translation(&mut self, translation: Vector3<Length>) {
        self.translation = translation;
    }

    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation;
    }

    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        Self {
            translation: self.translation + Vector3::new(x, y, z),
            rotation: self.rotation,
        }
    }

    pub fn rotate(self, rotation: Quaternion) -> Self {
        Self {
            translation: self.translation,
            rotation: self.rotation * rotation,
        }
    }

    pub fn rotate_on_axis(
        self,
        axis: Vector3<Number>,
        angle: Number,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        let mut transform: Transform3 = self.into();
        transform.rotate_on_axis(axis, angle, origin);
        transform.into()
    }

    /// Mirrors the transform along the given axis by applying a reflection matrix.
    /// Note: Assumes axis is normalized.
    pub fn mirror_along_axis(&mut self, axis: Vector3<Number>) {
        // Reflect the translation along the axis (reflection: R * translation)
        self.translation = self.translation.remap(axis);

        // Scale the rotation matrix by the reflection factor
        self.rotation = self.rotation.reflect_along_axis(u);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vector3::default(),
            rotation: Quaternion::default(),
        }
    }
}

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        let Transform {
            translation,
            rotation,
        } = value;
        BevyTransform {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: Vec3::new(1_f32, 1_f32, 1_f32),
        }
    }
}
