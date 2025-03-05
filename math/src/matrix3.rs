use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use serde::{Deserialize, Serialize};

use crate::quaternion::Quaternion;
use crate::vector3::Vector3;
use villagekit_number::{self as number, num, Number, Real};

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

    pub fn from_diagonal(diagonal: Vector3<Number>) -> Self {
        Self::from_rows(
            Vector3::new(diagonal.x, num!(0), num!(0)),
            Vector3::new(num!(0), diagonal.y, num!(0)),
            Vector3::new(num!(0), num!(0), diagonal.z),
        )
    }

    // TODO remove
    /// Note: Assumes axis is normalized.
    pub fn from_mirror_axis(axis: Vector3<Number>) -> Self {
        let identity = Self::identity();
        let outer_product = axis.outer(&axis);
        identity - num!(2) * outer_product
    }

    pub fn zero() -> Self {
        Self::from_cols(
            Vector3::new(num!(0), num!(0), num!(0)),
            Vector3::new(num!(0), num!(0), num!(0)),
            Vector3::new(num!(0), num!(0), num!(0)),
        )
    }

    pub fn identity() -> Self {
        Self::from_rows(
            Vector3::new(num!(1), num!(0), num!(0)),
            Vector3::new(num!(0), num!(1), num!(0)),
            Vector3::new(num!(0), num!(0), num!(1)),
        )
    }

    pub fn add_matrix3(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis + other.x_axis,
            y_axis: self.x_axis + other.y_axis,
            z_axis: self.x_axis + other.z_axis,
        }
    }

    pub fn sub_matrix3(self, other: Self) -> Self {
        Self {
            x_axis: self.x_axis - other.x_axis,
            y_axis: self.x_axis - other.y_axis,
            z_axis: self.x_axis - other.z_axis,
        }
    }

    pub fn mul_number(&self, other: Number) -> Self {
        Self {
            x_axis: self.x_axis * other,
            y_axis: self.y_axis * other,
            z_axis: self.z_axis * other,
        }
    }

    pub fn mul_vector3(&self, other: Vector3<Number>) -> Vector3<Number> {
        other.apply_matrix3(self)
    }

    pub fn mul_matrix3(self, other: Self) -> Self {
        Self {
            x_axis: self * other.x_axis,
            y_axis: self * other.y_axis,
            z_axis: self * other.z_axis,
        }
    }

    // TODO remove
    pub fn row(&self, index: usize) -> Vector3<Number> {
        match index {
            0 => Vector3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            1 => Vector3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            2 => Vector3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            _ => panic!("index out of bounds"),
        }
    }

    pub fn set(&mut self, matrix: Self) {
        self.x_axis = matrix.x_axis;
        self.y_axis = matrix.y_axis;
        self.z_axis = matrix.z_axis;
    }

    pub fn transpose(&self) -> Self {
        Self::from_rows(self.x_axis, self.y_axis, self.z_axis)
    }

    // TODO remove
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

    // TODO remove
    /// Creates a pure rotation matrix from axis + angle (Rodrigues' formula),
    /// by first building a Quaternion and then converting it.
    pub fn from_axis_angle(axis: Vector3<Number>, angle: Number) -> Self {
        let q = Quaternion::from_axis_angle(axis, angle);
        Self::from_quaternion(q)
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

    pub fn inverse(&self) -> Self {
        let determinant = self.determinant();

        if determinant == num!(0) {
            return Self::zero();
        }

        let determinant_inverse = num!(1) / determinant;

        let cofactor_1 = self.y_axis.cross(&self.z_axis);
        let cofactor_2 = self.z_axis.cross(&self.x_axis);
        let cofactor_3 = self.x_axis.cross(&self.y_axis);

        Self {
            x_axis: cofactor_1 * determinant_inverse,
            y_axis: cofactor_2 * determinant_inverse,
            z_axis: cofactor_3 * determinant_inverse,
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
    pub fn to_rotation_scale(&self) -> (Quaternion, Vector3<Number>) {
        // Step 1: measure the scale = length of each column
        let sx = self.x_axis.length();
        let sy = self.y_axis.length();
        let sz = self.z_axis.length();

        // Step 2: if determinant is negative, invert one axis's sign
        let det = self.determinant();
        let (sx, sy, sz) = if det < num!(0) {
            // e.g., flip X
            (-sx, sy, sz)
        } else {
            (sx, sy, sz)
        };

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
        let rot_x = self.x_axis * inv_sx;
        let rot_y = self.y_axis * inv_sy;
        let rot_z = self.z_axis * inv_sz;

        // Step 5: build a pure rotation matrix from that, then convert to Quaternion
        let rot_matrix = Matrix3::from_cols(rot_x, rot_y, rot_z);
        let quaternion: Quaternion = rot_matrix.into();

        let scale = Vector3::new(sx, sy, sz);

        (quaternion, scale)
    }
}

impl Display for Matrix3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x_axis, self.y_axis, self.z_axis)
    }
}

impl Add<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn add(self, rhs: Matrix3) -> Self::Output {
        self.add_matrix3(rhs)
    }
}

impl Sub<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, rhs: Matrix3) -> Self::Output {
        self.sub_matrix3(rhs)
    }
}

impl Mul<Number> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Number) -> Self::Output {
        self.mul_number(rhs)
    }
}

impl Mul<Matrix3> for Number {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        rhs.mul_number(self)
    }
}

impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        self.mul_matrix3(rhs)
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

        q.multipy_scalar(num!(0.5) / number::traits::Sqrt::sqrt(t))
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
