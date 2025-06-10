#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in radians.
pub struct Radians;

/// Type alias for an angle which has been specified in radians.
pub type AngleInRadians<T = f32> = Angle<Radians, T>;

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

impl<T> Angle<Radians, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in radians, from the specified value.
    pub fn radians(value: T) -> Angle<Radians, T> {
        AngleInRadians::new(value)
    }
}

impl<T> FromOther<T> for Angle<Radians, T>
where
    T: AngleValue,
{
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_radians(angle.borrow().value)
    }
}

impl<T> ConvertAngle<T> for Angle<Radians, T>
where
    T: AngleValue,
{
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::from_radians(self.value)
    }
}

use crate::{
    Angle, AngleUnit, AngleValue, ConvertAngle, Degrees, FromOther, Percentage, Rotations,
};
use core::borrow::Borrow;
