use serde::{Deserialize, Serialize};
use villagekit_number::Number;
use villagekit_unit::Length;

use crate::matrix3::Matrix3;
use crate::quaternion::Quaternion;
use crate::vector3::Vector3;

/// A 3D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Affine3 {
    matrix: Matrix3,
    translation: Vector3<Length>,
}

impl Affine3 {
    /// Creates an affine transform from translation, rotation (quaternion), and scale.
    ///
    /// Composition order: T(translation) * R(rotation) * S(scale).
    /// So `linear` = R * S, and `translation` is the translation part.
    pub fn from_translation_rotation_scale(
        translation: Vector3<Length>,
        rotation: Quaternion,
        scale: Vector3<Number>,
    ) -> Self {
        let rot_matrix = Matrix3::from_quaternion(rotation);
        let scale_matrix = Matrix3::from_diagonal(scale);
        let matrix = rot_matrix * scale_matrix; // R*S
        Self {
            matrix,
            translation,
        }
    }

    pub fn to_translation_rotation_scale(self) -> (Vector3<Length>, Quaternion, Vector3<Number>) {
        let Self {
            translation,
            matrix,
        } = self;
        let (rotation, scale) = matrix.to_rotation_scale();
        (translation, rotation, scale)
    }

    pub fn translate(self, x: Length, y: Length, z: Length) -> Self {
        Self {
            translation: self.translation + Vector3::new(x, y, z),
            ..self
        }
    }

    pub fn rotate_with_quaternion(self, quaternion: Quaternion) -> Self {
        Self {
            matrix: self.matrix * Matrix3::from_quaternion(quaternion),
            ..self
        }
    }

    pub fn scale(self, scale: Vector3<Number>) -> Self {
        Self {
            matrix: self.matrix * Matrix3::from_diagonal(scale),
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

        // Rotate the matrix part
        let matrix = self.matrix * rotation;

        Self {
            translation,
            matrix,
        }
    }

    /// Applies a change-of-basis transformation to this Transform.
    /// https://math.stackexchange.com/questions/628061/how-to-construct-change-of-basis-matrix/628075#628075
    pub fn change_basis(self, basis: Matrix3) -> Self {
        let transformation = Matrix3::identity().transpose() * basis;
        Self {
            translation: self.translation * transformation,
            matrix: self.matrix * transformation,
        }
    }
}
