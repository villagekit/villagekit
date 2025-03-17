pub mod macros;
pub mod traits;

use fastnum::{
    decimal::{Context, Decimal, ParseError},
    D128,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub use crate::traits::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Number(pub D128);

impl Number {
    const CONTEXT: Context = Context::default();

    pub const ZERO: Number = Number(Decimal::ZERO);
    pub const ONE: Number = Number(Decimal::ONE);
    pub const TWO: Number = Number(Decimal::TWO);

    pub const HALF: Number = num!(0.5);
    pub const QUARTER: Number = num!(0.25);

    pub const PI: Number = Number(Decimal::PI);
    pub const FRAC_1_PI: Number = Number(Decimal::FRAC_1_PI);
    pub const FRAC_2_PI: Number = Number(Decimal::FRAC_2_PI);
    pub const FRAC_PI_2: Number = Number(Decimal::FRAC_PI_2);
    pub const FRAC_PI_3: Number = Number(Decimal::FRAC_PI_3);
    pub const FRAC_PI_4: Number = Number(Decimal::FRAC_PI_4);

    pub const INFINITY: Number = Number(Decimal::INFINITY);
    pub const EPSILON: Number = Number(Decimal::EPSILON);

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        Ok(Self(Decimal::from_str(s, Self::CONTEXT)?))
    }

    pub const fn parse_unchecked(s: &str) -> Self {
        Self(Decimal::parse_str(s, Self::CONTEXT))
    }

    pub const fn add(self, rhs: Number) -> Self {
        Self(self.0.add(rhs.0))
    }

    pub const fn sub(self, rhs: Number) -> Self {
        Self(self.0.sub(rhs.0))
    }

    pub const fn mul(self, rhs: Number) -> Self {
        Self(self.0.mul(rhs.0))
    }

    pub const fn div(self, rhs: Number) -> Self {
        Self(self.0.div(rhs.0))
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        value.0.into()
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::ZERO
    }
}

// Special hash function to handle infinities and NaN,
//   which fastnum doesn't do.
impl Hash for Number {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let n = self.0;
        if n.is_infinite() {
            if n.is_positive() {
                "positive-infinity".hash(state)
            } else {
                "negative-infinity".hash(state)
            }
        } else if n.is_nan() {
            "not-a-number".hash(state)
        } else {
            n.hash(state)
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.div(rhs.0))
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.neg())
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Self::ZERO
    }
}

impl One for Number {
    fn one() -> Self {
        Self::ONE
    }
}

impl Sqrt for Number {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        Self(self.0.sqrt())
    }
}

impl Abs for Number {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self(self.0.abs())
    }
}

impl Trig for Number {
    type Output = Self;

    fn hypot(&self, other: Self) -> Self::Output {
        Self(self.0.hypot(other.0))
    }

    fn sin(&self) -> Self::Output {
        Self(self.0.sin())
    }

    fn cos(&self) -> Self::Output {
        Self(self.0.cos())
    }

    fn tan(&self) -> Self::Output {
        Self(self.0.tan())
    }

    fn sin_cos(&self) -> (Self::Output, Self::Output) {
        let (s, c) = self.0.sin_cos();
        (Self(s), Self(c))
    }
}

impl TrigInv for Number {
    type Output = Self;

    fn asin(&self) -> Self::Output {
        Self(self.0.asin())
    }

    fn acos(&self) -> Self::Output {
        Self(self.0.acos())
    }

    fn atan(&self) -> Self::Output {
        Self(self.0.atan())
    }

    fn atan2(&self, other: Self) -> Self::Output {
        Self(self.0.atan2(other.0))
    }
}

impl ApproxEq for Number {
    fn approx_eq(&self, rhs: &Number) -> bool {
        if self == rhs {
            return true;
        }

        (*self - *rhs).abs() <= Number::EPSILON
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected: Result<Number, ParseError> = Ok(num!(0.2));
        let actual = Number::parse("0.2");
        assert_eq!(expected, actual);

        let expected = num!(1.56);
        let actual = num!(1.30) * num!(1.20);
        assert_eq!(expected, actual);
    }
}
