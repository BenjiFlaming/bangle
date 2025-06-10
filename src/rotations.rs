#[derive(Clone, Copy, Debug, Default)]
/// An angle specified in rotations.
pub struct Rotations;

/// Type alias for an angle which has been specified in rotations.
pub type AngleInRotations<T = f32> = Angle<Rotations, T>;

impl<T> AngleUnit<T> for Rotations
where
    T: AngleValue,
{
    fn to_radians(value: T) -> Angle<Radians, T> {
        Angle::new(value.rotations_to_radians())
    }

    fn to_degrees(value: T) -> Angle<Degrees, T> {
        Angle::new(value.rotations_to_degrees())
    }

    fn to_rotations(value: T) -> Angle<Rotations, T> {
        Angle::new(value)
    }

    fn to_percentage(value: T) -> Angle<Percentage, T> {
        Angle::new(value.rotations_to_percentage())
    }

    fn from_radians(value: T) -> Angle<Self, T> {
        Angle::new(value.radians_to_rotations())
    }

    fn from_degrees(value: T) -> Angle<Self, T> {
        Angle::new(value.degrees_to_rotations())
    }

    fn from_rotations(value: T) -> Angle<Self, T> {
        Angle::new(value)
    }

    fn from_percentage(value: T) -> Angle<Self, T> {
        Angle::new(value.percentage_to_rotations())
    }
}

impl<T> Angle<Rotations, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in rotations, from the specified value.
    pub fn rotations(value: T) -> AngleInRotations<T> {
        AngleInRotations::new(value)
    }
}

impl<T> FromOther<T> for Angle<Rotations, T>
where
    T: AngleValue,
{
    fn from_other<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit<T>,
        T: AngleValue,
    {
        U::to_rotations(angle.borrow().value)
    }
}

impl<T> ConvertAngle<T> for Angle<Rotations, T>
where
    T: AngleValue,
{
    fn convert<U>(self) -> Angle<U, T>
    where
        U: AngleUnit<T>,
        T: AngleValue,
    {
        U::from_rotations(self.value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, ConvertAngle, Degrees, FromOther, Percentage, Radians};
use core::borrow::Borrow;
