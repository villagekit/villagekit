use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
use villagekit_number::{
    num,
    traits::{One, Sqrt, Zero},
    Number,
};

use crate::{Matrix3, Quaternion};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Vector3<N> {
    pub const fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
    }
}

impl<N: Display> Display for Vector3<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<N> Add for Vector3<N>
where
    N: Add<N, Output = N>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<N> Sub for Vector3<N>
where
    N: Sub<N, Output = N>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<N> Vector3<N>
where
    N: Mul<Number, Output = N>,
{
    pub fn multiply_scalar(self, n: Number) -> Self {
        Self::new(self.x * n, self.y * n, self.z * n)
    }
}

impl<N> Mul<Number> for Vector3<N>
where
    N: Mul<Number, Output = N>,
{
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        self.multiply_scalar(rhs)
    }
}

impl<N> Mul<Vector3<N>> for Number
where
    N: Mul<Number, Output = N>,
{
    type Output = Vector3<N>;

    fn mul(self, rhs: Vector3<N>) -> Self::Output {
        rhs.multiply_scalar(self)
    }
}

impl<N> Vector3<N>
where
    N: Div<Number, Output = N>,
{
    pub fn divide_scalar(self, n: Number) -> Self {
        Self::new(self.x / n, self.y / n, self.z / n)
    }
}

impl<N> Div<Number> for Vector3<N>
where
    N: Div<Number, Output = N>,
{
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        self.divide_scalar(rhs)
    }
}

impl<N> Div<Vector3<N>> for Number
where
    N: Div<Number, Output = N>,
{
    type Output = Vector3<N>;

    fn div(self, rhs: Vector3<N>) -> Self::Output {
        rhs.divide_scalar(self)
    }
}

impl<N> Vector3<N>
where
    N: Copy,
    // qxyz.cross(...)
    Number: Mul<N, Output = N>,
    N: Sub<N, Output = N>,
    // num!(2) * qxyz.cross(self)
    // q.w * t
    Number: Mul<Self, Output = Self>,
    // self + (...)
    Self: Add<Self, Output = Self>,
{
    pub fn multiply_quaternion(self, q: Quaternion) -> Self {
        let q = q.normalize();
        let q_xyz = Vector3::new(q.x, q.y, q.z);
        let t = Number::TWO * q_xyz.cross(&self);
        self + q.w * t + q_xyz.cross(&t)
    }
}

impl<N> Mul<Quaternion> for Vector3<N>
where
    N: Copy + Sub<N, Output = N>,
    Number: Mul<N, Output = N> + Mul<Self, Output = Self>,
    Self: Add<Self, Output = Self>,
{
    type Output = Self;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        self.multiply_quaternion(rhs)
    }
}

impl<N> Mul<Vector3<N>> for Quaternion
where
    N: Copy + Sub<N, Output = N>,
    Number: Mul<N, Output = N> + Mul<Vector3<N>, Output = Vector3<N>>,
    Vector3<N>: Add<Vector3<N>, Output = Vector3<N>>,
{
    type Output = Vector3<N>;

    fn mul(self, rhs: Vector3<N>) -> Self::Output {
        rhs.multiply_quaternion(self)
    }
}

impl<N> Vector3<N>
where
    N: Copy + Add<Output = N> + Mul,
    <N as Mul>::Output: Add<Output = <N as Mul>::Output> + Sqrt<Output = N>,
{
    pub fn length(self) -> N {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }
}

impl<N> Vector3<N>
where
    N: Copy,
    // self.length()
    N: Add<Output = N> + Mul,
    <N as Mul>::Output: Add<Output = <N as Mul>::Output> + Sqrt<Output = N>,
    // (...) / N::one()
    N: One + Div<N, Output = Number>,
    // (...) / length
    N: Div<Number, Output = N>,
{
    pub fn normalize(self) -> Self {
        let length = self.length() / N::one();
        if length == Number::ZERO {
            self
        } else {
            self / length
        }
    }
}

impl<A> Vector3<A> {
    pub fn dot<B>(&self, other: &Vector3<B>) -> <A as Mul<B>>::Output
    where
        B: Copy,
        A: Copy + Mul<B>,
        <A as Mul<B>>::Output: Add<Output = <A as Mul<B>>::Output>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<A> Vector3<A> {
    pub fn cross<B>(&self, other: &Vector3<B>) -> Vector3<<A as Mul<B>>::Output>
    where
        B: Copy,
        A: Copy + Mul<B>,
        <A as Mul<B>>::Output: Sub<Output = <A as Mul<B>>::Output>,
    {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Vector3<Number> {
    pub fn outer(&self, other: &Vector3<Number>) -> Matrix3 {
        Matrix3::from_cols(
            self.multiply_scalar(other.x),
            self.multiply_scalar(other.y),
            self.multiply_scalar(other.z),
        )
    }
}

impl<N> Vector3<N>
where
    N: Copy + Mul<Number, Output = N> + Add<N, Output = N>,
{
    pub fn apply_matrix3(&self, m: &Matrix3) -> Vector3<N> {
        let x = self.dot(&m.x_axis);
        let y = self.dot(&m.y_axis);
        let z = self.dot(&m.z_axis);
        Vector3 { x, y, z }
    }
}

impl<N> Mul<Matrix3> for Vector3<N>
where
    N: Copy + Mul<Number, Output = N> + Add<N, Output = N>,
{
    type Output = Vector3<N>;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        self.apply_matrix3(&rhs)
    }
}

impl<N> Mul<Vector3<N>> for Matrix3
where
    N: Copy + Mul<Number, Output = N> + Add<N, Output = N>,
{
    type Output = Vector3<N>;

    fn mul(self, rhs: Vector3<N>) -> Self::Output {
        rhs.apply_matrix3(&self)
    }
}

impl<N> Vector3<N>
where
    N: Copy + Mul<Number, Output = N> + Add<N, Output = N>,
{
    pub fn remap(self, basis: Matrix3) -> Self {
        basis.transpose() * self
    }
}

// https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector
// Note: Vector must be normalized
impl<N> Vector3<N>
where
    N: Copy,
    // self.dot(&axis)
    N: Mul<Number, Output = N> + Add<Output = N>,
    // self - (...)
    Self: Sub<Output = Self>,
    // num!(2) * (...)
    Number: Mul<N, Output = N>,
    // (...) * axis
    N: Mul<Vector3<Number>, Output = Self>,
{
    pub fn reflect(self, normal: Vector3<Number>) -> Self {
        self - num!(2) * self.dot(&normal) * normal
    }
}

impl<N> From<Vector3<N>> for glam::Vec3
where
    N: Into<f32>,
{
    fn from(value: Vector3<N>) -> Self {
        let Vector3 { x, y, z } = value;
        glam::Vec3::new(x.into(), y.into(), z.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(a.dot(&b), 32);
    }

    #[test]
    fn cross_product() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);
        // The cross product should be:
        // x = 2*6 - 3*5 = 12 - 15 = -3,
        // y = 3*4 - 1*6 = 12 - 6 = 6,
        // z = 1*5 - 2*4 = 5 - 8 = -3.
        let cross = a.cross(&b);
        assert_eq!(cross, Vector3::new(-3, 6, -3));
    }
}
