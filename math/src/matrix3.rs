use std::ops::{Add, Mul};

use serde::{Deserialize, Serialize};

use crate::quaternion::Quaternion;
use crate::vector3::Vector3;
use villagekit_number::{num, ops::Sqrt, Number};

/// A 3×3 matrix represented by three basis vectors (columns).
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Matrix3 {
    pub x_axis: Vector3<Number>,
    pub y_axis: Vector3<Number>,
    pub z_axis: Vector3<Number>,
}

impl Matrix3 {
    pub fn from_cols(
        x_axis: Vector3<Number>,
        y_axis: Vector3<Number>,
        z_axis: Vector3<Number>,
    ) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    pub fn from_rows(row1: Vector3<Number>, row2: Vector3<Number>, row3: Vector3<Number>) -> Self {
        Self {
            x_axis: Vector3::new(row1.x, row2.x, row3.x),
            y_axis: Vector3::new(row1.y, row2.y, row3.y),
            z_axis: Vector3::new(row1.z, row2.z, row3.z),
        }
    }

    pub fn identity() -> Self {
        Self::from_rows(
            Vector3::new(num!(1), num!(0), num!(0)),
            Vector3::new(num!(0), num!(1), num!(0)),
            Vector3::new(num!(0), num!(0), num!(1)),
        )
    }

    /// Creates a rotation matrix from a given unit Quaternion.
    pub fn from_quaternion(q: Quaternion) -> Self {
        // TODO Update to use the non-unit formula: https://en.wikipedia.org/wiki/Rotation_matrix#Quaternion

        let one = num!(1);
        let two = num!(2);

        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;

        // According to the standard formula ( https://en.wikipedia.org/wiki/Rotation_matrix#Quaternion ):
        //
        //   | 1 - 2yy - 2zz, 2xy - 2wz,     2xz + 2wy     |
        //   | 2xy + 2wz,     1 - 2xx - 2zz, 2yz - 2wx     |
        //   | 2xz - 2wy,     2yz + 2wx,     1 - 2xx - 2yy |
        let m00 = one - two * (yy + zz);
        let m01 = two * (xy - wz);
        let m02 = two * (xz + wy);

        let m10 = two * (xy + wz);
        let m11 = one - two * (xx + zz);
        let m12 = two * (yz - wx);

        let m20 = two * (xz - wy);
        let m21 = two * (yz + wx);
        let m22 = one - two * (xx + yy);

        Self {
            x_axis: Vector3::new(m00, m10, m20),
            y_axis: Vector3::new(m01, m11, m21),
            z_axis: Vector3::new(m02, m12, m22),
        }
    }

    /// Creates a pure rotation matrix from axis + angle (Rodrigues' formula),
    /// by first building a Quaternion and then converting it.
    pub fn from_axis_angle(axis: Vector3<Number>, angle: Number) -> Self {
        let q = Quaternion::from_axis_angle(axis, angle);
        Self::from_quaternion(q)
    }

    /// Creates a scale matrix (no rotation) from a scale vector.
    pub fn from_scale(scale: Vector3<Number>) -> Self {
        Self {
            x_axis: Vector3::new(scale.x, num!(0), num!(0)),
            y_axis: Vector3::new(num!(0), scale.y, num!(0)),
            z_axis: Vector3::new(num!(0), num!(0), scale.z),
        }
    }

    /// Compute the determinant of a Matrix3.
    pub fn determinant(&self) -> Number {
        let Self {
            x_axis,
            y_axis,
            z_axis,
        } = self;
        x_axis.dot(&y_axis.cross(z_axis))
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let c0 = Vector3::new(rhs.x_axis.x, rhs.x_axis.y, rhs.x_axis.z).apply_matrix3(&self);
        let c1 = Vector3::new(rhs.y_axis.x, rhs.y_axis.y, rhs.y_axis.z).apply_matrix3(&self);
        let c2 = Vector3::new(rhs.z_axis.x, rhs.z_axis.y, rhs.z_axis.z).apply_matrix3(&self);
        Self {
            x_axis: c0,
            y_axis: c1,
            z_axis: c2,
        }
    }
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::identity()
    }
}

// https://math.stackexchange.com/questions/893984/conversion-of-rotation-matrix-to-quaternion/3183435#3183435
impl From<Matrix3> for Quaternion {
    fn from(m: Matrix3) -> Self {
        let one = num!(1);

        // First, unpack the columns of Matrix3 into the usual
        // row/column notation mXY, where X is row and Y is column.
        //
        // Because x_axis is the first column, we map:
        //   m00 = x_axis.x, m10 = x_axis.y, m20 = x_axis.z
        //   m01 = y_axis.x, m11 = y_axis.y, m21 = y_axis.z
        //   m02 = z_axis.x, m12 = z_axis.y, m22 = z_axis.z

        let m00 = m.x_axis.x;
        let m10 = m.x_axis.y;
        let m20 = m.x_axis.z;

        let m01 = m.y_axis.x;
        let m11 = m.y_axis.y;
        let m21 = m.y_axis.z;

        let m02 = m.z_axis.x;
        let m12 = m.z_axis.y;
        let m22 = m.z_axis.z;

        let (t, q);

        #[allow(clippy::collapsible_else_if)]
        if m22 < num!(0) {
            if m00 > m11 {
                t = one + m00 - m11 - m22;
                q = Quaternion::new(t, m01 + m10, m20 + m02, m12 - m21);
            } else {
                t = one - m00 + m11 - m22;
                q = Quaternion::new(m01 + m10, t, m12 + m21, m20 - m02);
            }
        } else {
            if m00 < -m11 {
                t = one - m00 - m11 + m22;
                q = Quaternion::new(m20 + m02, m12 + m21, t, m01 - m10);
            } else {
                t = one + m00 + m11 + m22;
                q = Quaternion::new(m12 - m21, m20 - m02, m01 - m10, t);
            }
        }

        q.multipy_scalar(num!(0.5) / t.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use villagekit_number::{num, Number};

    use super::*;

    #[test]
    fn determinant_of_identity_matrix() {
        let expected = num!(1);
        let matrix = Matrix3::identity();
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant_of_singular_matrix() {
        let expected = num!(0);
        let matrix = Matrix3::from_rows(
            Vector3::new(num!(1), num!(2), num!(3)),
            Vector3::new(num!(4), num!(5), num!(6)),
            Vector3::new(num!(7), num!(8), num!(9)),
        );
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant_of_lower_triagular_matrix() {
        let expected = num!(7) * num!(2) * num!(5);
        let matrix = Matrix3::from_rows(
            Vector3::new(num!(7), num!(0), num!(0)),
            Vector3::new(num!(1), num!(2), num!(0)),
            Vector3::new(num!(3), num!(4), num!(5)),
        );
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant_of_upper_triangular_matrix() {
        let expected = num!(5) * num!(3) * num!(6);
        let matrix = Matrix3::from_rows(
            Vector3::new(num!(5), num!(2), num!(-1)),
            Vector3::new(num!(0), num!(3), num!(4)),
            Vector3::new(num!(0), num!(0), num!(6)),
        );
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant_of_random_positive_matrix() {
        let expected = num!(-33);
        let matrix = Matrix3::from_rows(
            Vector3::new(num!(3), num!(1), num!(4)),
            Vector3::new(num!(2), num!(0), num!(5)),
            Vector3::new(num!(7), num!(8), num!(6)),
        );
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }

    #[test]
    fn determinant_of_random_negative_matrix() {
        let expected = num!(139);
        let matrix = Matrix3::from_rows(
            Vector3::new(num!(-3), num!(1), num!(2)),
            Vector3::new(num!(0), num!(-4), num!(5)),
            Vector3::new(num!(7), num!(8), num!(-6)),
        );
        let actual = matrix.determinant();
        assert_eq!(expected, actual);
    }
}
