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

impl<T> FromAngle<T> for Angle<Degrees, T>
where
    T: AngleValue,
{
    fn from_angle<U>(angle: impl Borrow<Angle<U, T>>) -> Self
    where
        U: AngleUnit,
        T: AngleValue,
    {
        U::to_degrees(angle.borrow().value)
    }
}

use crate::{Angle, AngleUnit, AngleValue, Degrees, FromAngle};
use core::borrow::Borrow;
