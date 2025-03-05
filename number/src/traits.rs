pub use core::ops::*;
pub use num_traits::real::Real;

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

pub trait ApproxEq {
    fn approx_eq(&self, rhs: &Self) -> bool;
}
