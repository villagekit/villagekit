pub use core::ops::*;

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}
