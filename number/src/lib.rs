pub mod traits;

use fastnum::{
    decimal::{Context, Decimal, ParseError},
    D256,
};
use num_derive::{
    FromPrimitive, Neg, Num, NumCast, NumOps, One as OneDerive, Real, ToPrimitive,
    Zero as ZeroDerive,
};
pub use num_traits::real::Real;
use serde::{Deserialize, Serialize};
use traits::{One, Sqrt, Zero};

#[derive(
    Debug,
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
pub struct Number(pub D256);

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
        let d: D256 = Decimal::from_str(s, Self::CONTEXT)?;
        Ok(Self(d))
    }
}

impl From<Number> for f32 {
    fn from(value: Number) -> Self {
        value.0.into()
    }
}

impl Default for Number {
    fn default() -> Self {
        num!(0)
    }
}

impl Sqrt for Number {
    type Output = Number;

    fn sqrt(self) -> Self::Output {
        <Self as Real>::sqrt(self)
    }
}

impl Zero for Number {
    fn zero() -> Self {
        num!(0)
    }
}

impl One for Number {
    fn one() -> Self {
        num!(1)
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
