use serde::{Deserialize, Serialize};
use villagekit_unit::Length;

use crate::matrix3::Matrix3;
use crate::quaternion::Quaternion;
use crate::vector3::Vector3;
use villagekit_number::{num, Number, Real}; // for .sqrt(), .sin(), .cos(), etc.

////// A 3D affine transform storing:
///   - a 3×3 linear transformation (rotation, scale, shear) in `linear` (unitless)
///   - a translation in `Vector3<Length>` (with length units)
///
/// This is effectively a 4×4 matrix but kept in a smaller form with typed fields.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Transform3 {
    pub linear: Matrix3,              // rotation/scale/shear (unitless)
    pub translation: Vector3<Length>, // translation (with length units)
}

impl Transform3 {
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
        let scale_matrix = Matrix3::from_scale(scale);
        let linear = rot_matrix * scale_matrix; // R*S
        Self {
            linear,
            translation,
        }
    }

    /// Decompose this transform into (translation, quaternion, scale),
    /// following an approach similar to Three.js's `Matrix4.decompose`.
    ///
    /// The steps:
    /// 1) Extract scale from the column vectors' lengths (sx, sy, sz).
    /// 2) If the determinant is negative, flip one axis (e.g. sx = -sx).
    /// 3) Read the translation from `translation`.
    /// 4) Remove scale from `linear` to get a pure rotation+shear matrix.
    /// 5) Convert that to a `Quaternion`.
    /// 6) Return (translation, quaternion, scale).
    pub fn to_translation_rotation_scale(&self) -> (Vector3<Length>, Quaternion, Vector3<Number>) {
        // Step 1: measure the scale = magnitude of each column
        let sx = self.linear.x_axis.magnitude();
        let sy = self.linear.y_axis.magnitude();
        let sz = self.linear.z_axis.magnitude();

        // Step 2: if determinant is negative, invert one axis's sign
        let det = self.linear.determinant();
        let (sx, sy, sz) = if det < num!(0) {
            // e.g., flip X
            (-sx, sy, sz)
        } else {
            (sx, sy, sz)
        };

        let translation = self.translation;

        // Step 4: remove scale from each basis vector. This produces a
        // rotation+shear matrix that we can interpret as purely rotational
        // if there's no leftover shear. But if there's shear, we still produce a
        // best-fit quaternion.
        let inv_sx = if sx.abs() > num!(0) {
            num!(1) / sx
        } else {
            num!(0)
        };
        let inv_sy = if sy.abs() > num!(0) {
            num!(1) / sy
        } else {
            num!(0)
        };
        let inv_sz = if sz.abs() > num!(0) {
            num!(1) / sz
        } else {
            num!(0)
        };

        // unscaled columns
        let rot_x = self.linear.x_axis * inv_sx;
        let rot_y = self.linear.y_axis * inv_sy;
        let rot_z = self.linear.z_axis * inv_sz;

        // Step 5: build a pure rotation matrix from that, then convert to Quaternion
        let rot_matrix = Matrix3::new(rot_x, rot_y, rot_z);
        let quaternion: Quaternion = rot_matrix.into();

        let scale = Vector3::new(sx, sy, sz);

        (translation, quaternion, scale)
    }

    /// Rotate this transform around an arbitrary axis that passes through a given origin:
    ///
    /// 1) Translate so the origin is at (0, 0, 0).
    /// 2) Rotate around `direction` axis by `angle`.
    /// 3) Translate back.
    ///
    /// Then update our translation and linear accordingly.
    pub fn rotate_on_axis(
        &mut self,
        origin: Vector3<Length>,
        direction: Vector3<Number>,
        angle: Number,
    ) {
        // 1) Translate so that `origin` is at the origin.
        // We do: T -= origin
        // So effectively, we shift our current translation by -origin in world space
        self.translation = self.translation - origin;

        // 2) Construct the rotation matrix and apply it to the translation and linear.
        let rot = Matrix3::from_axis_angle(direction, angle);

        // Rotate the translation (in length units) about the new origin
        self.translation = rot.transform(self.translation);

        // Rotate the linear part
        self.linear = rot * self.linear;

        // 3) Translate back to the original pivot by adding `origin` again
        self.translation = self.translation + origin;
    }
}
