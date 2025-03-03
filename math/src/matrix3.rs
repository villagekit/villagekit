use std::ops::{Add, Mul};

use serde::{Deserialize, Serialize};

use crate::quaternion::Quaternion;
use crate::vector3::Vector3;
use villagekit_number::{num, ops::Sqrt, Number};

/// A 3×3 matrix represented by three basis vectors (columns):
///   [ x_axis  y_axis  z_axis ]
/// Each axis is a Vector3<Number>.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Matrix3 {
    pub x_axis: Vector3<Number>,
    pub y_axis: Vector3<Number>,
    pub z_axis: Vector3<Number>,
}

impl Matrix3 {
    /// Create a new Matrix3 from its three column vectors.
    pub fn new(x_axis: Vector3<Number>, y_axis: Vector3<Number>, z_axis: Vector3<Number>) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }

    /// Returns the 3×3 identity matrix.
    pub fn identity() -> Self {
        Self {
            x_axis: Vector3::new(num!(1), num!(0), num!(0)),
            y_axis: Vector3::new(num!(0), num!(1), num!(0)),
            z_axis: Vector3::new(num!(0), num!(0), num!(1)),
        }
    }

    /// Creates a rotation matrix from a given Quaternion.
    ///
    /// Standard conversion from quaternion (x, y, z, w) to a 3×3 matrix.
    pub fn from_quaternion(q: Quaternion) -> Self {
        // Normalize if desired (depends on whether your Quaternions are guaranteed normalized).
        // This sample does not explicitly normalize `q`.

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

        // According to the standard formula:
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

    /// Transforms a vector. Used for directions, normals, etc.
    pub fn transform<N>(&self, v: Vector3<N>) -> Vector3<N>
    where
        Number: Mul<N, Output = N>,
        N: Copy + Add<N, Output = N>,
    {
        // result.x = dot(self.x_axis, v)
        // result.y = dot(self.y_axis, v)
        // result.z = dot(self.z_axis, v)
        Vector3 {
            x: self.x_axis.x * v.x + self.y_axis.x * v.y + self.z_axis.x * v.z,
            y: self.x_axis.y * v.x + self.y_axis.y * v.y + self.z_axis.y * v.z,
            z: self.x_axis.z * v.x + self.y_axis.z * v.y + self.z_axis.z * v.z,
        }
    }

    /// Compute the determinant of a Matrix3.
    pub fn determinant(&self) -> Number {
        // Using the standard formula:
        // det = x_axis·(y_axis × z_axis)
        let cross_x = self.y_axis.y * self.z_axis.z - self.z_axis.y * self.y_axis.z;
        let cross_y = self.y_axis.z * self.z_axis.x - self.z_axis.z * self.y_axis.x;
        let cross_z = self.y_axis.x * self.z_axis.y - self.z_axis.x * self.y_axis.y;
        self.x_axis.x * cross_x + self.x_axis.y * cross_y + self.x_axis.z * cross_z
    }
}

/// Matrix multiplication (self * rhs).
/// Both are 3×3, so the result is also a 3×3.
impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let c0 = self.transform(Vector3::new(rhs.x_axis.x, rhs.x_axis.y, rhs.x_axis.z));
        let c1 = self.transform(Vector3::new(rhs.y_axis.x, rhs.y_axis.y, rhs.y_axis.z));
        let c2 = self.transform(Vector3::new(rhs.z_axis.x, rhs.z_axis.y, rhs.z_axis.z));
        Self {
            x_axis: c0,
            y_axis: c1,
            z_axis: c2,
        }
    }
}

/// Convert a pure rotation (and possibly shear) Matrix3 into a Quaternion.
/// (Adapted from the standard "matrix to quaternion" conversions, e.g. Three.js.)
impl From<Matrix3> for Quaternion {
    fn from(m: Matrix3) -> Self {
        let trace = m.x_axis.x + m.y_axis.y + m.z_axis.z;
        let half = num!(0.5);

        if trace > num!(0) {
            // S = sqrt(trace + 1) * 2; we factor out /2 in a single step
            let s = (trace + num!(1)).sqrt() * half;
            let w = s;
            let inv_s = half / s;
            let x = (m.y_axis.z - m.z_axis.y) * inv_s;
            let y = (m.z_axis.x - m.x_axis.z) * inv_s;
            let z = (m.x_axis.y - m.y_axis.x) * inv_s;
            Quaternion::new(x, y, z, w)
        } else {
            // If trace <= 0, pick the largest diagonal element to avoid numerical instability
            let (xx, yy, zz) = (m.x_axis.x, m.y_axis.y, m.z_axis.z);
            let max_axis = xx.max(yy.max(zz));

            if max_axis == xx {
                let s = (xx - yy - zz + num!(1)).sqrt() * half;
                let x = s;
                let inv_s = half / s;
                let y = (m.x_axis.y + m.y_axis.x) * inv_s;
                let z = (m.z_axis.x + m.x_axis.z) * inv_s;
                let w = (m.y_axis.z - m.z_axis.y) * inv_s;
                Quaternion::new(x, y, z, w)
            } else if max_axis == yy {
                let s = (-xx + yy - zz + num!(1)).sqrt() * half;
                let y = s;
                let inv_s = half / s;
                let x = (m.x_axis.y + m.y_axis.x) * inv_s;
                let z = (m.y_axis.z + m.z_axis.y) * inv_s;
                let w = (m.z_axis.x - m.x_axis.z) * inv_s;
                Quaternion::new(x, y, z, w)
            } else {
                // zz is largest
                let s = (-xx - yy + zz + num!(1)).sqrt() * half;
                let z = s;
                let inv_s = half / s;
                let x = (m.z_axis.x + m.x_axis.z) * inv_s;
                let y = (m.y_axis.z + m.z_axis.y) * inv_s;
                let w = (m.x_axis.y - m.y_axis.x) * inv_s;
                Quaternion::new(x, y, z, w)
            }
        }
    }
}
