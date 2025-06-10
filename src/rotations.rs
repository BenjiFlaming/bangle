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

impl<T> FromAngle<T> for Angle<Rotations, T>
where
    T: AngleValue,
{
    fn from_angle<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_rotations(angle.borrow().value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, FromAngle, Rotations};
use core::borrow::Borrow;
