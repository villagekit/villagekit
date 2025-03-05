use serde::{Deserialize, Serialize};
use std::ops::Mul;
use villagekit_number::{num, Number, Real};

use crate::vector3::Vector3;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Quaternion {
    pub x: Number,
    pub y: Number,
    pub z: Number,
    pub w: Number,
}

impl Quaternion {
    pub fn new(x: Number, y: Number, z: Number, w: Number) -> Self {
        Self { x, y, z, w }
    }
    pub fn from_axis_angle(axis: Vector3<Number>, angle: Number) -> Self {
        let axis = axis.normalize();
        let half_angle = Number::HALF * angle;
        let s = half_angle.sin();
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half_angle.cos(),
        }
    }

    pub fn multiply(self, other: Quaternion) -> Self {
        // from https://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/code/index.htm
        let (q1, q2) = (self, other);
        let x = q1.x * q2.w + q1.y * q2.z - q1.z * q2.y + q1.w * q2.x;
        let y = -q1.x * q2.z + q1.y * q2.w + q1.z * q2.x + q1.w * q2.y;
        let z = q1.x * q2.y - q1.y * q2.x + q1.z * q2.w + q1.w * q2.z;
        let w = -q1.x * q2.x - q1.y * q2.y - q1.z * q2.z + q1.w * q2.w;
        Self { x, y, z, w }
    }

    pub fn premultiply(self, other: Quaternion) -> Self {
        other.multiply(self)
    }

    pub fn multipy_scalar(self, n: Number) -> Self {
        let Self { x, y, z, w } = self;
        Quaternion::new(x * n, y * n, z * n, w * n)
    }

    pub fn length(self) -> Number {
        let Self { x, y, z, w } = self;
        (x * x + y * y + z * z + w * w).sqrt()
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        if length == Number::ZERO {
            Self::default()
        } else {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
                w: self.w / length,
            }
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            x: num!(0),
            y: num!(0),
            z: num!(0),
            w: num!(1),
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl From<Quaternion> for glam::Quat {
    fn from(value: Quaternion) -> Self {
        let Quaternion { x, y, z, w } = value;
        glam::Quat::from_xyzw(x.into(), y.into(), z.into(), w.into())
    }
}
