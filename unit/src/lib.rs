use std::ops::Add;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Canonical value is meter
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Length(f64);

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Default for Length {
    fn default() -> Self {
        Self(0.)
    }
}

pub struct Meter(pub f64);
impl From<Meter> for Length {
    fn from(value: Meter) -> Self {
        Self(value.0)
    }
}

pub struct Millimeter(pub f64);
impl From<Millimeter> for Length {
    fn from(value: Millimeter) -> Self {
        Self(value.0 * 0.001)
    }
}

pub struct Inch(pub f64);
impl From<Inch> for Length {
    fn from(value: Inch) -> Self {
        Self(value.0 * 0.0254)
    }
}

pub struct Foot(pub f64);
impl From<Foot> for Length {
    fn from(value: Foot) -> Self {
        Self(value.0 * 0.3048)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_ft() {
        let expected = Length(3.048);
        let actual: Length = Foot(10.).into();
        assert_eq!(expected, actual);
    }
}
