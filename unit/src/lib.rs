use std::ops::Add;

use villagekit_number::Number;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Canonical value is meter
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Length(Number);

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Default for Length {
    fn default() -> Self {
        Self(0.into())
    }
}

pub struct Meter(pub f64);
impl From<Meter> for Length {
    fn from(value: Meter) -> Self {
        Self(value.0.into())
    }
}

pub struct Millimeter(Number);
impl From<Millimeter> for Length {
    fn from(value: Millimeter) -> Self {
        Self(value.0 * 0.001.into())
    }
}

pub struct Inch(Number);
impl From<Inch> for Length {
    fn from(value: Inch) -> Self {
        Self(value.0 * 0.0254.into())
    }
}

pub struct Foot(Number);
impl From<Foot> for Length {
    fn from(value: Foot) -> Self {
        Self(value.0 * 0.3048.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_ft() {
        let expected = Length(3.048.into());
        let actual: Length = Foot(10.into()).into();
        assert_eq!(expected, actual);
    }
}
