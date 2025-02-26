use std::ops::{Add, Div, Mul, Sub};

use villagekit_number::{num, Number};

use crate::{Area, Length};

// Canonical value is meter^3
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Volume(pub Number);

impl Add for Volume {
    type Output = Volume;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Volume {
    type Output = Volume;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Number> for Volume {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<Number> for Volume {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<Length> for Volume {
    type Output = Area;

    fn div(self, rhs: Length) -> Self::Output {
        Area(self.0 / rhs.0)
    }
}

impl Div<Area> for Volume {
    type Output = Length;

    fn div(self, rhs: Area) -> Self::Output {
        Length(self.0 / rhs.0)
    }
}

impl Div<Volume> for Volume {
    type Output = Number;

    fn div(self, rhs: Volume) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self(0.into())
    }
}

pub struct CubicMeter(pub Number);
impl From<CubicMeter> for Volume {
    fn from(value: CubicMeter) -> Self {
        Self(value.0)
    }
}

pub struct CubicFoot(Number);
impl From<CubicFoot> for Volume {
    fn from(value: CubicFoot) -> Self {
        Self(value.0 * num!(0.02831685))
    }
}

#[cfg(test)]
mod tests {
    use villagekit_number::num;

    use super::*;

    #[test]
    fn convert_from_cubic_foot() {
        let expected = Volume(num!(1.699011));
        let actual: Volume = CubicFoot(num!(60)).into();
        assert_eq!(expected, actual);
    }
}
