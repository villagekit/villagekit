pub mod ops;

use fastnum::{
    dec128,
    decimal::{Context, Decimal, ParseError},
    D128,
};
use num_derive::{FromPrimitive, Neg, Num, NumCast, NumOps, One, Real, ToPrimitive, Zero};
use num_traits::real::Real;
use ops::Sqrt;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Zero,
    One,
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
        match Number::parse(concat!($(stringify!($ body)),*)) {
            Ok(n) => n,
            Err(e) => { panic!("{}", e) },
        }
    }}
}

impl Number {
    const CONTEXT: Context = Context::default();

    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let d: D128 = Decimal::from_str(s, Self::CONTEXT)?;
        Ok(Self(d))
    }
}

impl From<D128> for Number {
    fn from(value: D128) -> Self {
        Number(value)
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number(value.into())
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number(value.into())
    }
}

impl Default for Number {
    fn default() -> Self {
        Number(dec128!(0))
    }
}

impl Sqrt for Number {
    type Output = Number;

    fn sqrt(self) -> Self::Output {
        <Self as Real>::sqrt(self)
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
    }
}
