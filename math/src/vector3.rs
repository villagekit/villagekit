use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};
use villagekit_number::{
    num,
    traits::{One, Sqrt, Zero},
    Number,
};

use crate::Matrix3;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Vector3<N> {
    pub fn new(x: N, y: N, z: N) -> Self {
        Self { x, y, z }
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
    N: Copy + Add<Output = N> + Mul,
    <N as Mul>::Output: Add<Output = <N as Mul>::Output> + Sqrt<Output = N>,
{
    pub fn magnitude(self) -> N {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
    }
}

impl<N> Vector3<N>
where
    N: Copy,
    // self.magnitude()
    N: Add<Output = N> + Mul,
    <N as Mul>::Output: Add<Output = <N as Mul>::Output> + Sqrt<Output = N>,
    // (...) / N::one()
    N: One + Div<N, Output = Number>,
    // (...) / magnitude
    N: Div<Number, Output = N>,
{
    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude() / N::one();
        // TODO check for division by zero?
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
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

impl<N> Vector3<N>
where
    N: Copy + Mul,
    <N as Mul>::Output: Sub<Output = <N as Mul>::Output>,
{
    pub fn cross(&self, other: &Self) -> Vector3<<N as Mul>::Output> {
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
