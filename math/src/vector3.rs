use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};
use villagekit_number::{ops::Sqrt, Number};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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

impl<N> Mul<Number> for Vector3<N>
where
    N: Mul<Number, Output = N>,
{
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<N> Vector3<N>
where
    N: Copy + Add<N, Output = N> + Mul<N>,
    <N as Mul>::Output: Add<Output = <N as Mul>::Output> + Sqrt<Output = N>,
{
    pub fn magnitude(self) -> N {
        let Self { x, y, z } = self;
        (x * x + y * y + z * z).sqrt()
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
