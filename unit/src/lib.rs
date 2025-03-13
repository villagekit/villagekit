pub mod system;

pub use serde::{Deserialize, Serialize};
pub use system::*;
pub use villagekit_number::{
    num,
    traits::{Abs, ApproxEq, One, Sqrt, Zero},
    Number,
};

/// A trait implemented by all physical quantities.
pub trait Dimension {
    type CanonicalUnit: UnitOf<Dim = Self>;

    /// Converts the dimension to the given unit as number.
    #[inline]
    fn to<U: UnitOf<Dim = Self>>(&self) -> Number
    where
        Self: Sized,
    {
        (self.canonical() / U::CONVERSION_COEFFICIENT) - U::CONVERSION_CONSTANT
    }

    /// Creates a new dimension from the given number and unit.
    #[inline]
    fn from_scalar<U: UnitOf<Dim = Self>>(value: Number) -> Self
    where
        Self: Sized,
    {
        Self::from_canonical((value + U::CONVERSION_CONSTANT) * U::CONVERSION_COEFFICIENT)
    }

    /// Returns the canonical representation of the dimension.
    fn canonical(&self) -> Number;
    /// Creates a new dimension from the canonical representation.
    fn from_canonical(value: Number) -> Self;
}

#[macro_export]
macro_rules! to {
    ($dimension:ident in $unit:ty) => {
        $dimension.to::<$unit>()
    };
}

pub trait UnitOf {
    type Dim: Dimension;
    // To convert:
    //   canonical value = (this value + constant) * coefficient
    //   this value = (canonical value / coefficient) - constant
    const CONVERSION_COEFFICIENT: Number;
    const CONVERSION_CONSTANT: Number;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __measure_conversions {
    {} => {};
    {$self:ty,} => {};
    ($self:ident, Self * $rhs:ident => $output:ident, $($rest:tt)*) => {
        impl core::ops::Mul<$rhs> for $self {
            type Output = $output;
            fn mul(self, rhs: $rhs) -> Self::Output {
                use $crate::Dimension;
                $output::from_scalar::<<$output as Dimension>::CanonicalUnit>(self.canonical() * rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Self / $rhs:ident => $output:ident, $($rest:tt)*) => {
        impl core::ops::Div<$rhs> for $self {
            type Output = $output;
            fn div(self, rhs: $rhs) -> Self::Output {
                use $crate::Dimension;
                $output::from_scalar::<<$output as Dimension>::CanonicalUnit>(self.canonical() / rhs.canonical())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
    ($self:ident, Sqrt => $output:ident, $($rest:tt)*) => {
        impl villagekit_number::traits::Sqrt for $self {
            type Output = $output;
            fn sqrt(self) -> Self::Output {
                use $crate::Dimension;
                $output::from_scalar::<<$output as Dimension>::CanonicalUnit>(self.canonical().sqrt())
            }
        }

        $crate::__measure_conversions!($self, $($rest)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __unit_mult_imp {
    ($unit:ident, $dimension:ident, $($rhs:ident),*) => {
        $(
        )*
    };
}

/// A macro for creating a new unit type.
///
/// This macro creates a new unit type and implements multiplication with scalars on it.
#[macro_export]
macro_rules! unit_type {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default, $crate::Serialize, $crate::Deserialize)]
        $vis struct $unit;

        impl core::ops::Mul<$crate::Number> for $unit {
            type Output = $dimension;
            fn mul(self, rhs: $crate::Number) -> $dimension {
                use $crate::Dimension;
                $dimension::from_scalar::<$unit>(rhs)
            }
        }

        impl core::ops::Mul<$unit> for $crate::Number {
            type Output = $dimension;
            fn mul(self, _rhs: $unit) -> $dimension {
                use $crate::Dimension;
                $dimension::from_scalar::<$unit>(self)
            }
        }

        impl $unit {
            #[inline]
            pub fn from_scalar(value: $crate::Number) -> $dimension {
                use $crate::Dimension;
                $dimension::from_scalar::<Self>(value)
            }
        }
    };
}

/// A macro for creating a new unit type with simple conversions. Used internally by [`dimension!`](dimension)
///
/// Conversions are implemented by multiplying or dividing by a scalar value.
#[macro_export]
macro_rules! unit {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident = $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?
    ) => {
        $crate::unit_type!(
            $(#[$meta])*
            $vis $unit of dimension $dimension
        );

        $(
            impl $crate::UnitOf for $unit {
                type Dim = $dimension;

                const CONVERSION_COEFFICIENT: $crate::Number = $crate::num!($rhsper);
                const CONVERSION_CONSTANT: $crate::Number = $crate::num!(0);
            }
        )?
        $(
            impl $crate::UnitOf for $unit {
                type Dim = $dimension;

                const CONVERSION_COEFFICIENT: $crate::Number = $crate::num!($lhsper);
                const CONVERSION_CONSTANT: $crate::Number = $crate::num!(0);
            }
        )?
    };
}

/// A macro for creating a new dimension type and any simple associated unit types.
/// Associated unit types are parsed using similar syntax to the [`unit!`] macro.
#[macro_export]
macro_rules! dimension {
    (
        $(#[$meta:meta])*
        $vis:vis $name:ident {
            canonical: $canonical_unit:ident,

            $(
                $(#[$unit_meta:meta])*
                $unit:ident: $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?,
            )+
        } $(where {
            $($converts:tt)*
        })?
    ) => {
        // TODO remove inner pub
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, $crate::Serialize, $crate::Deserialize)]
        $vis struct $name(pub $crate::Number);

        impl $crate::Dimension for $name {
            type CanonicalUnit = $canonical_unit;

            #[inline]
            fn canonical(&self) -> $crate::Number {
                self.0.clone()
            }
            #[inline]
            fn from_canonical(value: $crate::Number) -> Self {
                Self(value)
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({:?} {})", stringify!($name), self.0, stringify!($canonical_unit))
            }
        }

        impl core::ops::Add<$name> for $name {
            type Output = $name;
            fn add(self, rhs: $name) -> $name {
                $name(self.0 + rhs.0)
            }
        }
        impl core::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: $name) {
                self.0 = self.0.clone() + rhs.0;
            }
        }
        impl core::ops::Sub<$name> for $name {
            type Output = $name;
            fn sub(self, rhs: $name) -> $name {
                $name(self.0 - rhs.0)
            }
        }
        impl core::ops::SubAssign<$name> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                self.0 = self.0.clone() - rhs.0;
            }
        }

        impl core::ops::Mul<$crate::Number> for $name {
            type Output = $name;
            fn mul(self, rhs: $crate::Number) -> $name {
                $name(self.0 * rhs)
            }
        }
        impl core::ops::MulAssign<$crate::Number> for $name {
            fn mul_assign(&mut self, rhs: $crate::Number) {
                self.0 = self.0.clone() * rhs;
            }
        }
        impl core::ops::Mul<$name> for $crate::Number {
            type Output = $name;
            fn mul(self, rhs: $name) -> $name {
                $name(self * rhs.0)
            }
        }

        impl core::ops::Div<$crate::Number> for $name {
            type Output = $name;
            fn div(self, rhs: $crate::Number) -> $name {
                $name(self.0 / rhs)
            }
        }
        impl core::ops::DivAssign<$crate::Number> for $name {
            fn div_assign(&mut self, rhs: $crate::Number) {
                self.0 = self.0.clone() / rhs;
            }
        }

        impl $crate::Zero for $name {
            fn zero() -> Self {
                $name($crate::Number::ZERO)
            }
        }

        impl $crate::One for $name {
            fn one() -> Self {
                $name($crate::Number::ONE)
            }
        }

        impl $crate::Abs for $name {
            type Output = $name;

            fn abs(self) -> Self {
                $name(self.0.abs())
            }
        }

        impl $crate::ApproxEq for $name {
            fn approx_eq(&self, other: &Self) -> bool {
                self.0.approx_eq(&other.0)
            }
        }

        $(
            $crate::unit!(
                $(#[$unit_meta])*
                $vis $unit of dimension $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )*

        $(
            $crate::__measure_conversions!($name, $($converts)*);
        )?
    };
}
