/// An angle which has been specified in radians.
pub type AngleInRadians<T = f32> = Angle<Radians, T>;

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

use crate::{Angle, AngleUnit, AngleValue, ConvertAngle, FromOther, Radians};
use core::borrow::Borrow;
