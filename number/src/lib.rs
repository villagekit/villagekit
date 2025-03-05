pub mod traits;

use std::fmt::{Debug, Display};

use fastnum::{
    decimal::{Context, Decimal, ParseError},
    D128,
};
use num_derive::{
    FromPrimitive, Neg, Num, NumCast, NumOps, One as OneDerive, Real, ToPrimitive,
    Zero as ZeroDerive,
};
pub use num_traits::real::Real;
use serde::{Deserialize, Serialize};
use traits::{ApproxEq, One, Sqrt, Zero};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ZeroDerive,
    OneDerive,
    NumOps,
    Num,
    ToPrimitive,
    FromPrimitive,
    NumCast,
    Neg,
    Real,
)]
pub struct Number(pub D128);

#[macro_export]
macro_rules! num {
    ($($ body:tt)*) => {{
        const __NUMBER: Number = Number::parse_unchecked(concat!($(stringify!($ body)),*));
        __NUMBER
    }}
}

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

impl Sqrt for Number {
    type Output = Number;

    fn sqrt(self) -> Self::Output {
        <Self as Real>::sqrt(self)
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
