/// An angle which has been specified in rotations.
pub type AngleInRotations<T = f32> = Angle<Rotations, T>;

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
        U: AngleUnit,
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
        U: AngleUnit,
        T: AngleValue,
    {
        U::from_rotations(self.value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, ConvertAngle, FromOther, Rotations};
use core::borrow::Borrow;
