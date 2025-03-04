use bevy_transform::components::Transform as BevyTransform;
use serde::{Deserialize, Serialize};
use villagekit_math::{Affine3, Matrix3, Quaternion, Vector3};
use villagekit_number::Number;
use villagekit_unit::Length;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Transform(Affine3);

impl Transform {
    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        Self(self.0.translate(x, y, z))
    }

    pub fn rotate_with_quaternion(self, quaternion: Quaternion) -> Self {
        Self(self.0.rotate_with_quaternion(quaternion))
    }

    pub fn scale(self, scale: Vector3<Number>) -> Self {
        Self(self.0.scale(scale))
    }

    pub fn rotate(
        self,
        axis: Vector3<Number>,
        angle: Number,
        origin: Option<Vector3<Length>>,
    ) -> Self {
        Self(self.0.rotate(axis, angle, origin))
    }

    pub fn change_basis(self, basis: Matrix3) -> Self {
        Self(self.0.change_basis(basis))
    }
}

impl From<Transform> for BevyTransform {
    fn from(value: Transform) -> Self {
        let (translation, rotation, scale) = value.0.to_translation_rotation_scale();
        BevyTransform {
            translation: translation.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }
}
