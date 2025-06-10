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

impl<T> FromAngle<T> for Angle<Radians, T>
where
    T: AngleValue,
{
    fn from_angle<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_radians(angle.borrow().value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, FromAngle, Radians};
use core::borrow::Borrow;
