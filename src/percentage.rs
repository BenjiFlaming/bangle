#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in percentage.
pub struct Percentage;

/// Type alias for an angle which has been specified in rotations.
pub type AngleInPercentage<T = f32> = Angle<Percentage, T>;

impl<T> AngleUnit<T> for Percentage
where
    T: AngleValue,
{
    fn to_radians(value: T) -> Angle<Radians, T> {
        Angle::new(value.percentage_to_radians())
    }

    fn to_degrees(value: T) -> Angle<Degrees, T> {
        Angle::new(value.percentage_to_degrees())
    }

    fn to_rotations(value: T) -> Angle<Rotations, T> {
        Angle::new(value.percentage_to_rotations())
    }

    fn to_percentage(value: T) -> Angle<Percentage, T> {
        Angle::new(value)
    }

    fn from_radians(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_percentage())
    }

    fn from_degrees(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_percentage())
    }

    fn from_rotations(value: T) -> Angle<Self, T> {
        Angle::new(value.rotations_to_percentage())
    }

    fn from_percentage(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }
}

impl<T> Angle<Percentage, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in percentage, from the specified value.
    pub fn percentage(value: T) -> AngleInPercentage<T> {
        AngleInPercentage::new(value)
    }
}

impl<T> FromOther<T> for Angle<Percentage, T>
where
    T: AngleValue,
{
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit<T>,
        T: AngleValue,
    {
        U::to_percentage(angle.borrow().value)
    }
}

impl<T> ConvertAngle<T> for Angle<Percentage, T>
where
    T: AngleValue,
{
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit<T>,
        T: AngleValue,
    {
        U::from_percentage(self.value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, ConvertAngle, Degrees, FromOther, Radians, Rotations};
use core::borrow::Borrow;
