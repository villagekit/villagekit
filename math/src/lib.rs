use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul};
use villagekit_number::{num, ops::Sqrt, Number};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Vector3<N> {
    x: N,
    y: N,
    z: N,
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Quaternion {
    x: Number,
    y: Number,
    z: Number,
    w: Number,
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

#[cfg(test)]
mod tests {
    use villagekit_number::num;
    use villagekit_unit::Length;

    use super::*;

    #[test]
    fn magnitude_of_vector_with_units() {
        let expected = Length(num!(10) * num!(3).sqrt());
        let v = Vector3::new(Length(num!(10)), Length(num!(10)), Length(num!(10)));
        let actual = v.magnitude();
        assert_eq!(expected, actual);
    }
}
