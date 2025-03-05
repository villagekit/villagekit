// TODO split up and implement all traits in num-traits::real::Real
//   https://docs.rs/num-traits/0.2.19/src/num_traits/real.rs.html

pub use core::ops::{Add, Div, Mul, Neg, Sub};

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

pub trait AbsSub {
    type Output;

    fn abs_sub(self, other: Self) -> Self::Output;
}

impl<T> AbsSub for T
where
    T: Zero + PartialOrd + Sub<T, Output = T>,
{
    type Output = T;

    fn abs_sub(self, other: Self) -> Self::Output {
        if self.le(&other) {
            Self::zero()
        } else {
            self - other
        }
    }
}

pub trait ApproxEq {
    fn approx_eq(&self, rhs: &Self) -> bool;
}

pub trait Trig {
    type Output;

    fn hypot(&self, other: Self) -> Self::Output;
    fn sin(&self) -> Self::Output;
    fn cos(&self) -> Self::Output;
    fn tan(&self) -> Self::Output;
    fn asin(&self) -> Self::Output;
    fn acos(&self) -> Self::Output;
    fn atan(&self) -> Self::Output;
    fn atan2(&self, other: Self) -> Self::Output;
    fn sin_cos(&self) -> (Self::Output, Self::Output);
}

pub trait Hyp {
    type Output;

    fn sinh(&self) -> Self::Output;
    fn cosh(&self) -> Self::Output;
    fn tanh(&self) -> Self::Output;
    fn asinh(&self) -> Self::Output;
    fn acosh(&self) -> Self::Output;
    fn atanh(&self) -> Self::Output;
}

// TODO min_value
// TODO min_positive_value
// TODO epsilon
// TODO max_value
//
// TODO floor
// TODO ceil
// TODO round
// TODO trunc
// TODO fract
//
// TODO signum
// TODO is_sign_positive
// TODO is_sign_negative
//
// TODO mul_add
// TODO recip
// TODO powi
// TODO powf
//
// TODO exp
// TODO exp2
//
// TODO ln
// TODO log
// TODO log2
// TODO log10
//
// TODO to_radians
// TODO to_degrees
//
// TODO min
// TODO max
//
// TODO cbrt
// TODO exp_ml
