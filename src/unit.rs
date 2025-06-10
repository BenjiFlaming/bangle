/// This marker trait allows different "types" of angles to be defined.
pub trait AngleUnit: Clone + Copy + Debug + Default {
    /// Converts the supplied value, from this unit's type, into radians.
    fn to_radians<T: AngleValue>(value: T) -> Angle<Radians, T>;

    /// Converts the supplied value, from this unit's type, into degrees.
    fn to_degrees<T: AngleValue>(value: T) -> Angle<Degrees, T>;

    /// Converts the supplied value, from this unit's type, into rotations.
    fn to_rotations<T: AngleValue>(value: T) -> Angle<Rotations, T>;

    /// Converts the supplied value, from this unit's type, into percentage.
    fn to_percentage<T: AngleValue>(value: T) -> Angle<Percentage, T>;

    /// Converts the supplied value, in radians, into this unit's type.
    fn from_radians<T: AngleValue>(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in degrees, into this unit's type.
    fn from_degrees<T: AngleValue>(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in rotations, into this unit's type.
    fn from_rotations<T: AngleValue>(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in percentage, into this unit's type.
    fn from_percentage<T: AngleValue>(value: T) -> Angle<Self, T>;
}

#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in radians.
pub struct Radians;
impl AngleUnit for Radians {
    fn to_radians<T: AngleValue>(value: T) -> Angle<Radians, T> {
        Angle::new(value)
    }

    fn to_degrees<T: AngleValue>(value: T) -> Angle<Degrees, T> {
        Angle::new(value.radians_to_degrees())
    }

    fn to_rotations<T: AngleValue>(value: T) -> Angle<Rotations, T> {
        Angle::new(value.radians_to_rotations())
    }

    fn to_percentage<T: AngleValue>(value: T) -> Angle<Percentage, T> {
        Angle::new(value.radians_to_percentage())
    }

    fn from_radians<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }

    fn from_degrees<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.degrees_to_radians())
    }

    fn from_rotations<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.rotations_to_radians())
    }

    fn from_percentage<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.percentage_to_radians())
    }
}

#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in degrees.
pub struct Degrees;
impl AngleUnit for Degrees {
    fn to_radians<T: AngleValue>(value: T) -> Angle<Radians, T> {
        Angle::new(value.degrees_to_radians())
    }

    fn to_degrees<T: AngleValue>(value: T) -> Angle<Degrees, T> {
        Angle::new(value)
    }

    fn to_rotations<T: AngleValue>(value: T) -> Angle<Rotations, T> {
        Angle::new(value.degrees_to_rotations())
    }

    fn to_percentage<T: AngleValue>(value: T) -> Angle<Percentage, T> {
        Angle::new(value.degrees_to_percentage())
    }

    fn from_radians<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_degrees())
    }

    fn from_degrees<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }

    fn from_rotations<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.rotations_to_degrees())
    }

    fn from_percentage<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.percentage_to_degrees())
    }
}

#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in rotations.
pub struct Rotations;
impl AngleUnit for Rotations {
    fn to_radians<T: AngleValue>(value: T) -> Angle<Radians, T> {
        Angle::new(value.rotations_to_radians())
    }

    fn to_degrees<T: AngleValue>(value: T) -> Angle<Degrees, T> {
        Angle::new(value.rotations_to_degrees())
    }

    fn to_rotations<T: AngleValue>(value: T) -> Angle<Rotations, T> {
        Angle::new(value)
    }

    fn to_percentage<T: AngleValue>(value: T) -> Angle<Percentage, T> {
        Angle::new(value.rotations_to_percentage())
    }

    fn from_radians<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_rotations())
    }

    fn from_degrees<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.degrees_to_rotations())
    }

    fn from_rotations<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }

    fn from_percentage<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.percentage_to_rotations())
    }
}

#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in percentage.
pub struct Percentage;
impl AngleUnit for Percentage {
    fn to_radians<T: AngleValue>(value: T) -> Angle<Radians, T> {
        Angle::new(value.percentage_to_radians())
    }

    fn to_degrees<T: AngleValue>(value: T) -> Angle<Degrees, T> {
        Angle::new(value.percentage_to_degrees())
    }

    fn to_rotations<T: AngleValue>(value: T) -> Angle<Rotations, T> {
        Angle::new(value.percentage_to_rotations())
    }

    fn to_percentage<T: AngleValue>(value: T) -> Angle<Percentage, T> {
        Angle::new(value)
    }

    fn from_radians<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_percentage())
    }

    fn from_degrees<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_percentage())
    }

    fn from_rotations<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value.rotations_to_percentage())
    }

    fn from_percentage<T: AngleValue>(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }
}

use crate::{Angle, AngleValue};
use core::fmt::Debug;
