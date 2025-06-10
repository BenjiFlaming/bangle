/// An angle which has been specified in rotations.
pub type AngleInPercentage<T = f32> = Angle<Percentage, T>;

impl<T> Angle<Percentage, T>
where
    T: AngleValue,
{
    /// Constructs a new angle, represented in percentage, from the specified value.
    pub fn percentage(value: T) -> AngleInPercentage<T> {
        AngleInPercentage::new(value)
    }
}

impl<T> FromAngle<T> for Angle<Percentage, T>
where
    T: AngleValue,
{
    fn from_angle<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_percentage(angle.borrow().value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, FromAngle, Percentage};
use core::borrow::Borrow;
