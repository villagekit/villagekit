use villagekit_number::{Number, Sqrt};

use crate::{dimension, system_qty_macro};

dimension!(
    /// Represents a distance.
    ///
    /// Canonically represented in meters.
    pub Length {
        canonical: Meters,

        /// Represents the millimeter unit of length.
        Millimeters: 1000.0 per canonical,
        /// Represents the centimeter unit of length.
        Centimeters: 100.0 per canonical,
        /// Represents the meter unit of length.
        /// This is the standard SI unit of length.
        Meters: 1.0 per canonical,
        /// Represents the kilometer unit of length.
        Kilometers: per 1000.0 canonical,

        /// Represents the inch unit of length.
        Inches: 39.3700787401575 per canonical,
        /// Represents the foot unit of length.
        Feet: per 0.3048 canonical,
        /// Represents the yard unit of length.
        Yards: per 0.9144 canonical,
        /// Represents the mile unit of length.
        Miles: per 1609.344 canonical,
        /// Represents the nautical mile unit of length.
        NauticalMiles: per 1852.0 canonical,
    } where {
        // Self / Time => LinearVelocity in MetersPerSecond,
        // Self * Force => Energy in Joules,
        Self * Length => Area,
        Self * Area => Volume,
    }
);

impl Sqrt for Length {
    type Output = Number;

    fn sqrt(self) -> Self::Output {
        self.0.sqrt()
    }
}

dimension!(
    pub Area {
        canonical: SquareMeters,

        /// Represents the square millimeter unit of area.
        SquareMillimeters: 1_000_000.0 per canonical,
        /// Represents the square centimeter unit of area.
        SquareCentimeters: 10_000.0 per canonical,
        /// Represents the square meter unit of area.
        /// This is the standard SI unit of area.
        SquareMeters: 1.0 per canonical,
        /// Represents the square kilometer unit of area.
        SquareKilometers: per 1_000_000.0 canonical,

        /// Represents the square inch unit of area.
        SquareInches: 1550.0031000062 per canonical,
        /// Represents the square foot unit of area.
        SquareFeet: 10.7639104167097 per canonical,
        /// Represents the square yard unit of area.
        SquareYards: per 0.83612736 canonical,
        /// Represents the acre unit of area.
        Acres: per 4046.8564224 canonical,
    } where {
        Self / Length => Length,
        Self * Length => Volume,
    }
);

impl Sqrt for Area {
    type Output = Length;

    fn sqrt(self) -> Self::Output {
        Length(self.0.sqrt())
    }
}

dimension!(
    pub Volume {
        canonical: CubicMeters,

        /// Represents the cubic millimeter unit of volume.
        Milliliters: 1_000_000.0 per canonical,
        /// Represents the cubic centimeter unit of volume.
        Liters: 1000.0 per canonical,

        /// Represents the cubic meter unit of volume.
        CubicMillimeters: 1_000_000_000.0 per canonical,
        /// Represents the cubic meter unit of volume.
        CubicCentimeters: 1_000_000.0 per canonical,
        /// Represents the cubic meter unit of volume.
        CubicMeters: 1.0 per canonical,
        /// Represents the cubic kilometer unit of volume.
        CubicKilometers: per 1_000_000_000.0 canonical,

        /// Represents the cubic inch unit of volume.
        CubicInches: 61023.7440947323 per canonical,
        /// Represents the cubic foot unit of volume.
        CubicFeet: 35.3146667214886 per canonical,
        /// Represents the cubic yard unit of volume.
        CubicYards: 1.30795061931439 per canonical,

        /// Represents the fluid ounce unit of volume.
        FluidOunces: 33814.022701843 per canonical,
        /// Represents the pint unit of volume.
        Pints: 2113.37641886519 per canonical,
        /// Represents the quart unit of volume.
        Quarts: 1056.68820943259 per canonical,
        /// Represents the gallon unit of volume.
        Gallons: 264.172052358148 per canonical,

    } where {
        Self / Length => Area,
        Self / Area => Length,
    }
);

impl Sqrt for Volume {
    type Output = Area;

    fn sqrt(self) -> Self::Output {
        Area(self.0.sqrt())
    }
}

system_qty_macro! {
    qty,
    Length {
        m   => Meters,
        cm  => Centimeters,
        mm  => Millimeters,
        km  => Kilometers,
        in  => Inches,
        ft  => Feet,
        yd  => Yards,
        mi  => Miles,
        nmi => NauticalMiles,
    }
    Area {
        m2  => SquareMeters,
        cm2 => SquareCentimeters,
        mm2 => SquareMillimeters,
        km2 => SquareKilometers,
        in2 => SquareInches,
        ft2 => SquareFeet,
        yd2 => SquareYards,
        ac  => Acres,
    }
    Volume {
        m3   => CubicMeters,
        cm3  => CubicCentimeters,
        mm3  => CubicMillimeters,
        km3  => CubicKilometers,
        ml   => Milliliters,
        l    => Liters,
        in3  => CubicInches,
        ft3  => CubicFeet,
        yd3  => CubicYards,
        floz => FluidOunces,
        pt   => Pints,
        qt   => Quarts,
        gal  => Gallons,
    }
}
