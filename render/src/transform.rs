use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Matrix3, Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::Length;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Transform {
    translation: Vector3<Length>,
    linear: Matrix3,
}

impl Transform {
    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        Self {
            translation: self.translation + Vector3::new(x, y, z),
            ..self
        }
    }

    pub fn rotate_with_quaternion(self, quaternion: Quaternion) -> Self {
        Self {
            linear: self.linear * Matrix3::from_quaternion(quaternion),
            ..self
        }
    }

    pub fn scale(self, scale: Vector3<Number>) -> Self {
        Self {
            linear: self.linear * Matrix3::from_diagonal(scale),
            ..self
        }
    }

    /// Rotate this transform around an arbitrary axis that passes through a given origin.
    pub fn rotate(
        self,
        axis: Vector3<Number>,
        angle: Number,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        let origin = origin.unwrap_or_default();
        let rotation = Matrix3::from_axis_angle(axis, angle);

        // - Translate so that `origin` is at the origin.
        // - Rotate the translation (in length units) about the new origin
        // - Translate back to the original pivot by adding `origin` again
        let translation = (self.translation - origin) * rotation + origin;

        // Rotate the linear part
        let linear = self.linear * rotation;

        Self {
            translation,
            linear,
        }
    }

    /// Applies a change-of-basis transformation to this Transform.
    pub fn change_basis(self, basis: Matrix3) -> Self {
        Self {
            translation: basis * self.translation,
            linear: basis * self.linear * basis.inverse(),
        }
    }
}

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        let Transform {
            translation,
            linear,
        } = value;
        let (rotation, scale) = linear.to_rotation_scale();
        BevyTransform {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}
