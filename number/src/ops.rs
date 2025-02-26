pub use core::ops::*;

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}
