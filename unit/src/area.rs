use std::ops::{Add, Div, Mul, Sub};

use villagekit_number::Number;

use crate::{Length, Volume};

// Canonical value is meter^2
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Area(pub Number);

impl Add for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Area {
    type Output = Area;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<Number> for Area {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul<Length> for Area {
    type Output = Volume;

    fn mul(self, rhs: Length) -> Self::Output {
        Volume(self.0 * rhs.0)
    }
}

impl Div<Number> for Area {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Div<Length> for Area {
    type Output = Length;

    fn div(self, rhs: Length) -> Self::Output {
        Length(self.0 / rhs.0)
    }
}

impl Div<Area> for Area {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Default for Area {
    fn default() -> Self {
        Self(0.into())
    }
}

pub struct SquareMeter(pub Number);
impl From<SquareMeter> for Area {
    fn from(value: SquareMeter) -> Self {
        Self(value.0)
    }
}

pub struct SquareFoot(Number);
impl From<SquareFoot> for Area {
    fn from(value: SquareFoot) -> Self {
        Self(value.0 * 0.09290304.into())
    }
}

#[cfg(test)]
mod tests {
    use villagekit_number::num;

    use super::*;

    #[test]
    fn convert_from_square_foot() {
        let expected = Area(num!(2.7870912));
        let actual: Area = SquareFoot(num!(30)).into();
        assert_eq!(expected, actual);
    }
}
