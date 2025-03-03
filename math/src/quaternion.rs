use serde::{Deserialize, Serialize};
use std::ops;
use villagekit_number::{num, Number, Real};

use crate::vector3::Vector3;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
        let Vector3 { x, y, z } = axis;
        let half_angle = angle / num!(2);
        let s = half_angle.sin();
        let w = s.cos();
        Self { x, y, z, w }
    }
    pub fn multiply(self, other: Quaternion) -> Self {
        Quaternion::multiply_quaternions(self, other)
    }
    pub fn premultiply(self, other: Quaternion) -> Self {
        Quaternion::multiply_quaternions(other, self)
    }
    pub fn multiply_quaternions(a: Quaternion, b: Quaternion) -> Self {
        let x = a.x * b.w + a.w * b.x + a.y * b.z - a.z * b.y;
        let y = a.y * b.w + a.w * b.y + a.z * b.x - a.x * b.z;
        let z = a.z * b.w + a.w * b.z + a.x * b.y - a.y * b.x;
        let w = a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z;
        Self { x, y, z, w }
    }
    pub fn multipy_scalar(self, n: Number) -> Self {
        let Self { x, y, z, w } = self;
        Quaternion::new(x * n, y * n, z * n, w * n)
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

impl From<Quaternion> for glam::Quat {
    fn from(value: Quaternion) -> Self {
        let Quaternion { x, y, z, w } = value;
        glam::Quat::from_xyzw(x.into(), y.into(), z.into(), w.into())
    }
}
