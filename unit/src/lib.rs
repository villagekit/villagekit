pub mod dimensions;

pub use dimensions::*;
pub use serde::{Deserialize, Serialize};
pub use villagekit_number::{num, traits::Sqrt, Number};

/// A trait implemented by all physical quantities.
pub trait Dimension {
    type CanonicalUnit: UnitOf<Self>;

    /// Converts the dimension to the given unit.
    #[inline]
    fn to<U: UnitOf<Self>>(&self) -> Number
    where
        Self: Sized,
    {
        U::from_canonical(self.canonical())
    }

    /// Creates a new dimension from the given scalar and unit.
    #[inline]
    fn from_scalar<U: UnitOf<Self>>(value: Number) -> Self
    where
        Self: Sized,
    {
        Self::from_canonical(U::to_canonical(value))
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

pub trait UnitOf<M: Dimension + ?Sized> {
    /// Converts a scalar value from the canonical unit to unit of `Self`.
    fn from_canonical(canonical: Number) -> Number;
    /// Converts a scalar value from the unit of `Self` to the canonical unit.
    fn to_canonical(converted: Number) -> Number;
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
macro_rules! simple_unit {
    (
        $(#[$meta:meta])*
        $vis:vis $unit:ident of dimension $dimension:ident = $($rhsper:literal per canonical)? $(per $lhsper:literal canonical)?
    ) => {
        $crate::unit_type!(
            $(#[$meta])*
            $vis $unit of dimension $dimension
        );

        $(
            impl $crate::UnitOf<$dimension> for $unit {
                #[inline]
                fn from_canonical(canonical: $crate::Number) -> $crate::Number {
                    canonical * $crate::num!($rhsper)
                }
                #[inline]
                fn to_canonical(converted: $crate::Number) -> $crate::Number {
                    converted / $crate::num!($rhsper)
                }
            }
        )?
        $(
            impl $crate::UnitOf<$dimension> for $unit {
                #[inline]
                fn from_canonical(canonical: $crate::Number) -> $crate::Number {
                    canonical /  $crate::num!($lhsper)
                }
                #[inline]
                fn to_canonical(converted: $crate::Number) -> $crate::Number {
                    converted *  $crate::num!($lhsper)
                }
            }
        )?
    };
}

/// A macro for creating a new dimension type and any simple associated unit types.
/// Associated unit types are parsed using similar syntax to the [`simple_unit!`] macro.
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

        $(
            $crate::simple_unit!(
                $(#[$unit_meta])*
                $vis $unit of dimension $name = $($rhsper per canonical)? $(per $lhsper canonical)?
            );
        )*

        $(
            $crate::__measure_conversions!($name, $($converts)*);
        )?
    };
}
