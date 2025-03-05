use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
use villagekit_number::{
    num,
    traits::{Abs, ApproxEq, One, Sqrt, Zero},
    Number,
};

use crate::{Area, Volume};

// Canonical value is meter
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Length(pub Number);

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Number> for Length {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Length> for Number {
    type Output = Length;

    fn mul(self, rhs: Length) -> Self::Output {
        Length(self * rhs.0)
    }
}

impl Mul<Length> for Length {
    type Output = Area;

    fn mul(self, rhs: Self) -> Self::Output {
        Area(self.0 * rhs.0)
    }
}

impl Mul<Area> for Length {
    type Output = Volume;

    fn mul(self, rhs: Area) -> Self::Output {
        Volume(self.0 * rhs.0)
    }
}

impl Div<Number> for Length {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<Length> for Length {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Zero for Length {
    fn zero() -> Self {
        Self(Number::zero())
    }
}

impl One for Length {
    fn one() -> Self {
        Self(Number::one())
    }
}

impl Sqrt for Length {
    type Output = Number;

    fn sqrt(self) -> Self::Output {
        self.0.sqrt()
    }
}

impl Abs for Length {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self(self.0.abs())
    }
}

impl ApproxEq for Length {
    fn approx_eq(&self, rhs: &Self) -> bool {
        self.0.approx_eq(&rhs.0)
    }
}

impl Default for Length {
    fn default() -> Self {
        Self(num!(0))
    }
}

impl From<Length> for f32 {
    fn from(value: Length) -> Self {
        value.0.into()
    }
}

pub struct Meter(pub Number);
impl From<Meter> for Length {
    fn from(value: Meter) -> Self {
        Self(value.0)
    }
}

pub struct Millimeter(pub Number);
impl From<Millimeter> for Length {
    fn from(value: Millimeter) -> Self {
        Self(value.0 * num!(0.001))
    }
}

pub struct Inch(pub Number);
impl From<Inch> for Length {
    fn from(value: Inch) -> Self {
        Self(value.0 * num!(0.0254))
    }
}

pub struct Foot(pub Number);
impl From<Foot> for Length {
    fn from(value: Foot) -> Self {
        Self(value.0 * num!(0.3048))
    }
}

#[cfg(test)]
mod tests {
    use villagekit_number::num;

    use super::*;

    #[test]
    fn convert_from_foot() {
        let expected = Length(num!(3.048));
        let actual: Length = Foot(num!(10)).into();
        assert_eq!(expected, actual);
    }
}
