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

/// An angle which has been specified in degrees.
pub type AngleInDegrees<T = f32> = Angle<Degrees, T>;

impl<T> Angle<Degrees, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in degrees, from the specified value.
    pub fn degrees(value: T) -> AngleInDegrees<T> {
        AngleInDegrees::new(value)
    }
}

impl<T> FromOther<T> for Angle<Degrees, T>
where
    T: AngleValue,
{
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_degrees(angle.borrow().value)
    }
}

impl<T> ConvertAngle<T> for Angle<Degrees, T>
where
    T: AngleValue,
{
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::from_degrees(self.value)
    }
}

use crate::{
    Angle, AngleUnit, AngleValue, ConvertAngle, FromOther, Percentage, Radians, Rotations,
};
use core::borrow::Borrow;
