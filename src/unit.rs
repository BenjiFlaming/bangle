/// A collection of conversion functions, which allow converting betwen different angle units.
pub trait AngleUnit<T = f32>: Clone + Copy + Debug + Default
where
    T: AngleValue,
{
    /// Converts the supplied value, from this unit's type, into radians.
    fn to_radians(value: T) -> Angle<Radians, T>;

    /// Converts the supplied value, from this unit's type, into degrees.
    fn to_degrees(value: T) -> Angle<Degrees, T>;

    /// Converts the supplied value, from this unit's type, into rotations.
    fn to_rotations(value: T) -> Angle<Rotations, T>;

    /// Converts the supplied value, from this unit's type, into percentage.
    fn to_percentage(value: T) -> Angle<Percentage, T>;

    /// Converts the supplied value, in radians, into this unit's type.
    fn from_radians(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in degrees, into this unit's type.
    fn from_degrees(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in rotations, into this unit's type.
    fn from_rotations(value: T) -> Angle<Self, T>;

    /// Converts the supplied value, in percentage, into this unit's type.
    fn from_percentage(value: T) -> Angle<Self, T>;
}

/// Allows conversion to this angle unit from any other unit.
pub trait FromOther<T = f32>: Sized {
    /// Converts an angle into this type.
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit<T>,
        T: AngleValue;
}

/// Allows this unit to be converted to any other unit.
pub trait ConvertAngle<T = f32> {
    /// Converts this angle into another type.
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit<T>,
        T: AngleValue;
}

use crate::{Angle, AngleValue, Degrees, Percentage, Radians, Rotations};
use core::{borrow::Borrow, fmt::Debug};
